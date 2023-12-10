use crate::{
    external_class::ExternalClass,
    sys::{lean_io_result_mk_error, lean_io_result_mk_ok},
};

pub(crate) enum LeanIoResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> ExternalClass for LeanIoResult<T, E>
where
    T: ExternalClass,
    E: ExternalClass,
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
