use std::{ffi::c_void, sync::OnceLock};

use polars::{prelude::NamedFromOwned, series::Series};

use crate::{
    data_type::DataType,
    sys::{
        b_lean_obj_arg, convert_string, lean_alloc_external, lean_array_object,
        lean_external_class, lean_get_external_data, lean_obj_res, lean_register_external_class,
        lean_unbox, LeanArray, SyncPtr, lean_unbox_uint64, lean_is_scalar, lean_scalar_to_int64, lean_unbox_float,
    },
};

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

/// # Safety
/// The lean function must pass `Array dt.asType` as the second argument.
// TODO: when we introduce complex types like List, DataType will be represented as lean_ctor_object.
#[no_mangle]
unsafe extern "C" fn polars_lean_series_from_array(
    dt: DataType,
    array: b_lean_obj_arg,
) -> lean_obj_res {
    assert_eq!((*array).m_tag(), LeanArray);
    let array: *const lean_array_object = array.cast();
    let objs = (*array).m_data.as_slice((*array).m_size);

    macro_rules! boxed_uint_to_series {
        ($ty:ty, $data:expr) => {{
            let v = $data
                .iter()
                .map(|&o| {
                    let v = lean_unbox(o);
                    v as $ty
                })
                .collect::<Vec<$ty>>();
            Series::from_vec("series", v)
        }};
    }
    macro_rules! int_to_series {
        ($ty:ty, $data:expr) => {{
            let v = $data.iter()
                .map(|&o| {
                    if lean_is_scalar(o) {
                        // TODO: handle overflow
                        lean_scalar_to_int64(o) as $ty
                    } else {
                        todo!("handle bigint")
                    }
                })
                .collect::<Vec<$ty>>();
            Series::from_vec("series", v)
        }};
    }

    let series = match dt {
        DataType::UInt8 => boxed_uint_to_series!(u8, objs),
        DataType::UInt16 => boxed_uint_to_series!(u16, objs),
        DataType::UInt32 => boxed_uint_to_series!(u32, objs),
        DataType::UInt64 => {
            let v = objs
                .iter()
                .map(|&o| lean_unbox_uint64(o))
                .collect::<Vec<u64>>();
            Series::from_vec("series", v)
        },
        DataType::Int8 => int_to_series!(i8, objs),
        DataType::Int16 => int_to_series!(i16, objs),
        DataType::Int32 => int_to_series!(i32, objs),
        DataType::Int64 => int_to_series!(i64, objs),
        DataType::Float64 => {
            let v = objs
                .iter()
                .map(|&o| lean_unbox_float(o))
                .collect::<Vec<f64>>();
            Series::from_vec("series", v)
        },
    };
    let series = Box::into_raw(Box::new(series));
    let cls = series_external_class();
    lean_alloc_external(cls, series.cast())
}

/// # Safety
/// The lean function must pass a valid series
#[no_mangle]
pub unsafe extern "C" fn polars_lean_print_series(series: b_lean_obj_arg) -> lean_obj_res {
    let series: *mut Series = lean_get_external_data(series);
    let s = series.as_ref().unwrap().to_string();
    convert_string(s.as_str())
}
