use crate::{
    external_class::ToLean,
    sys::{lean_io_result_mk_error, lean_io_result_mk_ok},
};

pub(crate) enum LeanIoResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> ToLean for LeanIoResult<T, E>
where
    T: ToLean,
    E: ToLean,
{
    fn to_lean(self) -> *mut crate::sys::lean_object {
        unsafe {
            match self {
                Self::Ok(t) => lean_io_result_mk_ok(t.to_lean()),
                Self::Err(e) => lean_io_result_mk_error(e.to_lean()),
            }
        }
    }
}

pub(crate) trait ToIoResult<T, E> {
    fn to_io_result(self) -> LeanIoResult<T, E>;
}

impl<T, E> ToIoResult<T, E> for Result<T, E> {
    fn to_io_result(self) -> LeanIoResult<T, E> {
        match self {
            Ok(t) => LeanIoResult::Ok(t),
            Err(e) => LeanIoResult::Err(e),
        }
    }
}
