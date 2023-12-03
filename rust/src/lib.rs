use std::{ffi::c_void, sync::OnceLock};

use polars::{prelude::NamedFrom, series::Series};
use sys::{
    convert_string, lean_alloc_external, lean_external_class, lean_obj_res,
    lean_register_external_class, SyncPtr, lean_obj_arg,
};

use crate::sys::{b_lean_obj_arg, lean_get_external_data};

mod sys;

static SERIES_EXTERNAL_CLASS: OnceLock<SyncPtr<lean_external_class>> = OnceLock::new();

fn series_external_class() -> *mut lean_external_class {
    extern "C" fn finalize(ptr: *mut c_void) {
        drop(unsafe { Box::from_raw(ptr as *mut Series) });
    }
    extern "C" fn foreach(_: *mut c_void, _: b_lean_obj_arg) {
        // do nothing as series do not contain any lean objects
    }
    SERIES_EXTERNAL_CLASS
        .get_or_init(|| {
            let external_class =
                unsafe { lean_register_external_class(Some(finalize), Some(foreach)) };
            SyncPtr(external_class)
        })
        .0
}

#[no_mangle]
pub extern "C" fn give_me_a_series(_unit: lean_obj_arg) -> lean_obj_res {
    // TODO: can we save this double indirection?
    let series = Box::new(Series::new("foo", vec![1, 2, 3]));
    let series = Box::into_raw(series);
    let external_class = series_external_class();
    unsafe { lean_alloc_external(external_class, series.cast()) }
}

/// # Safety
/// The lean function must pass a valid series
#[no_mangle]
pub unsafe extern "C" fn polars_lean_print_series(series: b_lean_obj_arg) -> lean_obj_res {
    let series: *mut Series = lean_get_external_data(series);
    let s = series.as_ref().unwrap().to_string();
    convert_string(s.as_str())
}
