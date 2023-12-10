use std::ffi::c_void;

use crate::sys::{
    b_lean_obj_arg, lean_alloc_ctor, lean_alloc_external, lean_ctor_set, lean_external_class,
    lean_object,
};

pub(crate) trait ExternalClass: Sized {
    fn to_lean(self) -> *mut lean_object;
}

pub(crate) trait BoxedExternalClass: Sized {
    fn get_external_class() -> *mut lean_external_class;

    fn to_lean(self) -> *mut lean_object {
        let ptr = Box::into_raw(Box::new(self));
        let cls = Self::get_external_class();
        unsafe { lean_alloc_external(cls, ptr.cast()) }
    }
}

impl<T> ExternalClass for T
where
    T: BoxedExternalClass,
{
    fn to_lean(self) -> *mut lean_object {
        BoxedExternalClass::to_lean(self)
    }
}

impl<T, E> ExternalClass for Result<T, E>
where
    T: ExternalClass,
    E: ExternalClass,
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
