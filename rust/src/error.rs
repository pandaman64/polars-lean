use polars::error::PolarsError;

use crate::{
    external_class::declare_simple_external_class,
    sys::{b_lean_obj_arg, convert_string, lean_get_external_data, lean_obj_res},
};

declare_simple_external_class!(PolarsError);

/// # Safety
/// The first argument must be a `PolarsError`.
#[no_mangle]
unsafe extern "C" fn polars_lean_polars_error_to_string(err: b_lean_obj_arg) -> lean_obj_res {
    let err: *mut PolarsError = lean_get_external_data(err);
    let err = err.as_ref().unwrap();
    convert_string(err.to_string().as_str())
}
