use polars::{frame::DataFrame, series::Series};

use crate::{
    external_class::ExternalClass,
    sys::{
        b_lean_obj_arg, convert_string, lean_array_object, lean_get_external_data, lean_obj_res,
        LeanArray,
    },
};

crate::external_class::declare_simple_external_class!(DataFrame);

/// # Safety
/// The lean function must pass `Array Series` as the first argument.
#[no_mangle]
unsafe extern "C" fn polars_lean_data_frame_from_series_array(
    array: b_lean_obj_arg,
) -> lean_obj_res {
    assert_eq!((*array).m_tag(), LeanArray);

    let array: *const lean_array_object = array.cast();
    let objs = (*array).m_data.as_slice((*array).m_size);

    let series = objs
        .iter()
        .map(|&o| {
            let series_ptr: *mut Series = lean_get_external_data(o);
            series_ptr.as_ref().unwrap().clone()
        })
        .collect::<Vec<_>>();

    DataFrame::new(series).to_lean()
}

/// # Safety
/// The lean function must pass `DataFrame` as the first argument.
#[no_mangle]
unsafe extern "C" fn polars_lean_print_data_frame(df: b_lean_obj_arg) -> lean_obj_res {
    let df: *mut DataFrame = lean_get_external_data(df);
    let s = df.as_ref().unwrap().to_string();
    convert_string(s.as_str())
}
