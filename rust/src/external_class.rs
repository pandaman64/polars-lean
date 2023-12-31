use std::ffi::c_void;

use crate::sys::{
    b_lean_obj_arg, lean_alloc_ctor, lean_alloc_external, lean_ctor_set, lean_dec,
    lean_external_class, lean_get_external_data, lean_is_exclusive, lean_obj_arg, lean_obj_res,
    lean_object, lean_set_external_class, lean_set_external_data,
};

pub(crate) trait ToLean: Sized {
    fn to_lean(self) -> *mut lean_object;
}

impl ToLean for *mut lean_object {
    fn to_lean(self) -> *mut lean_object {
        self
    }
}

/// Given an external object, clone the underlying data and apply the function.
unsafe fn apply_external_cloned<T, U, F>(o: lean_obj_arg, f: F) -> lean_obj_res
where
    F: FnOnce(T) -> U,
    T: Clone,
    U: ToLean,
{
    let ptr: *mut T = lean_get_external_data(o);
    let this = ptr.as_ref().unwrap().clone();

    lean_dec(o);

    f(this).to_lean()
}

pub(crate) trait BoxedExternalClass: Sized {
    fn get_external_class() -> *mut lean_external_class;

    /// # Safety
    /// The function must not panic or panic = "abort".
    unsafe fn try_consume<T, E, F>(o: lean_obj_arg, f: F) -> lean_obj_res
    where
        Self: Clone,
        F: FnOnce(Self) -> Result<T, E>,
        T: BoxedExternalClass,
        E: BoxedExternalClass,
    {
        if lean_is_exclusive(o) {
            // Reuse the storage for lean_external_object, wrapping it in `Except`.
            let ptr: *mut Self = lean_get_external_data(o);
            let this = Box::from_raw(ptr);
            match f(*this) {
                Ok(t) => {
                    lean_set_external_class(o, T::get_external_class());
                    lean_set_external_data(o, Box::into_raw(Box::new(t)));
                    Ok(o)
                }
                Err(e) => {
                    lean_set_external_class(o, E::get_external_class());
                    lean_set_external_data(o, Box::into_raw(Box::new(e)));
                    Err(o)
                }
            }
            .to_lean()
        } else {
            // We don't have exclusive access to the object, so we need to clone it.
            apply_external_cloned(o, f)
        }
    }

    /// # Safety
    /// The function must not panic or panic = "abort".
    unsafe fn consume<T, F>(o: lean_obj_arg, f: F) -> lean_obj_res
    where
        Self: Clone,
        F: FnOnce(Self) -> T,
        T: BoxedExternalClass,
    {
        if lean_is_exclusive(o) {
            // Reuse the storage for lean_external_object
            let ptr: *mut Self = lean_get_external_data(o);
            let this = Box::from_raw(ptr);
            let result_ptr = Box::into_raw(Box::new(f(*this)));
            let cls = T::get_external_class();

            lean_set_external_class(o, cls);
            lean_set_external_data(o, result_ptr);
            o
        } else {
            // We don't have exclusive access to the object, so we need to clone it.
            apply_external_cloned(o, f)
        }
    }

    /// # Safety
    /// The function must not panic or panic = "abort".
    unsafe fn take_self<F>(o: lean_obj_arg, f: F) -> lean_obj_res
    where
        Self: Clone,
        F: FnOnce(Self) -> Self,
    {
        if lean_is_exclusive(o) {
            // Reuse the storage for lean_external_object and Box.
            let ptr: *mut Self = lean_get_external_data(o);
            ptr.write(f(ptr.read()));
            o
        } else {
            // We don't have exclusive access to the object, so we need to clone it.
            apply_external_cloned(o, f)
        }
    }
}

impl<T> ToLean for T
where
    T: BoxedExternalClass,
{
    fn to_lean(self) -> *mut lean_object {
        let ptr = Box::into_raw(Box::new(self));
        let cls = Self::get_external_class();
        unsafe { lean_alloc_external(cls, ptr.cast()) }
    }
}

impl<T, E> ToLean for Result<T, E>
where
    T: ToLean,
    E: ToLean,
{
    // Result is represented by an `Except E T`
    fn to_lean(self) -> *mut lean_object {
        let (tag, content) = match self {
            // Except.error comes first
            Err(e) => (0, e.to_lean()),
            Ok(t) => (1, t.to_lean()),
        };
        unsafe {
            let r = lean_alloc_ctor(tag, 1, 0);
            lean_ctor_set(r, 0, content);
            r
        }
    }
}

pub(crate) extern "C" fn simple_external_class_finalize<T>(ptr: *mut c_void) {
    drop(unsafe { Box::from_raw(ptr as *mut T) });
}

pub(crate) extern "C" fn simple_external_class_foreach(_: *mut c_void, _: b_lean_obj_arg) {
    // do nothing as the type don't reference any lean objects
}

/// Helpers for types without backreferences to Lean objects
///
/// We manage the underlying Rust object using a raw pointer backed by a Box.
macro_rules! declare_simple_external_class {
    ($ty:ty) => {
        static EXTERNAL_CLASS: std::sync::OnceLock<
            crate::sys::SyncPtr<crate::sys::lean_external_class>,
        > = std::sync::OnceLock::new();

        impl crate::external_class::BoxedExternalClass for $ty {
            fn get_external_class() -> *mut crate::sys::lean_external_class {
                let finalize = crate::external_class::simple_external_class_finalize::<$ty>;
                let foreach = crate::external_class::simple_external_class_foreach;
                EXTERNAL_CLASS
                    .get_or_init(|| {
                        let external_class = unsafe {
                            crate::sys::lean_register_external_class(Some(finalize), Some(foreach))
                        };
                        crate::sys::SyncPtr(external_class)
                    })
                    .0
            }
        }
    };
}

pub(crate) use declare_simple_external_class;
