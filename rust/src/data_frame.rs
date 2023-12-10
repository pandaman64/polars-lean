use polars::{
    error::PolarsResult,
    frame::DataFrame,
    io::{csv::CsvReader, SerReader},
    series::Series,
};

use crate::{
    external_class::ExternalClass,
    io::LeanIoResult,
    sys::{
        b_lean_obj_arg, convert_string, lean_array_object, lean_get_external_data, lean_obj_res,
        lean_string_to_str, LeanArray,
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

#[no_mangle]
unsafe extern "C" fn polars_lean_data_frame_read_csv(s: b_lean_obj_arg) -> lean_obj_res {
    // TODO: support options
    fn read_csv(path: &str) -> PolarsResult<DataFrame> {
        CsvReader::from_path(path)?.finish()
    }
    let path = lean_string_to_str(s);
    match read_csv(path) {
        Ok(df) => LeanIoResult::Ok(df),
        Err(e) => LeanIoResult::Err(e),
    }
    .to_lean()
}
