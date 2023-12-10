use polars::{
    chunked_array::ops::SortOptions,
    lazy::frame::{LazyCsvReader, LazyFileListReader, LazyFrame},
};

use crate::{
    external_class::{declare_simple_external_class, BoxedExternalClass, ToLean},
    io::ToIoResult,
    sys::{
        b_lean_obj_arg, convert_string, lean_get_external_data, lean_obj_arg, lean_obj_res,
        lean_string_to_str,
    },
};

declare_simple_external_class!(LazyFrame);

#[no_mangle]
unsafe extern "C" fn polars_lean_lazy_frame_sort(
    lf: lean_obj_arg,
    by_column: b_lean_obj_arg,
) -> lean_obj_res {
    let by_column = lean_string_to_str(by_column);
    LazyFrame::take_self(lf, |lf| lf.sort(by_column, SortOptions::default()))
}

// TODO: I guess this is IO
#[no_mangle]
unsafe extern "C" fn polars_lean_lazy_frame_collect(lf: lean_obj_arg) -> lean_obj_res {
    LazyFrame::try_consume(lf, LazyFrame::collect)
}

#[no_mangle]
unsafe extern "C" fn polars_lean_lazy_frame_describe_plan(lf: b_lean_obj_arg) -> lean_obj_res {
    let lf: *mut LazyFrame = lean_get_external_data(lf);
    let s = lf.as_ref().unwrap().describe_plan();
    convert_string(s.as_str())
}

#[no_mangle]
unsafe extern "C" fn polars_lean_lazy_frame_describe_optimized_plan(
    lf: b_lean_obj_arg,
) -> lean_obj_res {
    let lf: *mut LazyFrame = lean_get_external_data(lf);
    let e = lf
        .as_ref()
        .unwrap()
        .describe_optimized_plan()
        .map(|s| convert_string(s.as_str()));
    e.to_lean()
}

#[no_mangle]
unsafe extern "C" fn polars_lean_lazy_frame_scan_csv(s: b_lean_obj_arg) -> lean_obj_res {
    let path = lean_string_to_str(s);
    LazyCsvReader::new(path).finish().to_io_result().to_lean()
}
