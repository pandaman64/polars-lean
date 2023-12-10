use polars::{prelude::NamedFromOwned, series::Series};

use crate::{
    data_type::DataType,
    external_class::BoxedExternalClass,
    sys::{
        b_lean_obj_arg, convert_string, lean_array_object, lean_get_external_data, lean_is_scalar,
        lean_obj_res, lean_scalar_to_int64, lean_string_to_str, lean_unbox, lean_unbox_float,
        lean_unbox_uint64, LeanArray, LeanString,
    },
};

crate::external_class::declare_simple_external_class!(Series);

/// # Safety
/// The lean function must pass `String` as the second and `Array dt.asType` as the third argument.
// TODO: when we introduce complex types like List, DataType will be represented as lean_ctor_object.
#[no_mangle]
unsafe extern "C" fn polars_lean_series_from_array(
    dt: DataType,
    name: b_lean_obj_arg,
    array: b_lean_obj_arg,
) -> lean_obj_res {
    assert_eq!((*array).m_tag(), LeanArray);
    assert_eq!((*name).m_tag(), LeanString);
    let name = lean_string_to_str(name);

    let array: *const lean_array_object = array.cast();
    let objs = (*array).m_data.as_slice((*array).m_size);

    macro_rules! boxed_uint_to_series {
        ($ty:ty, $data:expr, $name:expr) => {{
            let v = $data
                .iter()
                .map(|&o| {
                    let v = lean_unbox(o);
                    v as $ty
                })
                .collect::<Vec<$ty>>();
            Series::from_vec($name, v)
        }};
    }
    macro_rules! int_to_series {
        ($ty:ty, $data:expr, $name:expr) => {{
            let v = $data
                .iter()
                .map(|&o| {
                    if lean_is_scalar(o) {
                        // TODO: handle overflow
                        lean_scalar_to_int64(o) as $ty
                    } else {
                        todo!("handle bigint")
                    }
                })
                .collect::<Vec<$ty>>();
            Series::from_vec($name, v)
        }};
    }

    let series = match dt {
        DataType::UInt8 => boxed_uint_to_series!(u8, objs, name),
        DataType::UInt16 => boxed_uint_to_series!(u16, objs, name),
        DataType::UInt32 => boxed_uint_to_series!(u32, objs, name),
        DataType::UInt64 => {
            let v = objs
                .iter()
                .map(|&o| lean_unbox_uint64(o))
                .collect::<Vec<u64>>();
            Series::from_vec(name, v)
        }
        DataType::Int8 => int_to_series!(i8, objs, name),
        DataType::Int16 => int_to_series!(i16, objs, name),
        DataType::Int32 => int_to_series!(i32, objs, name),
        DataType::Int64 => int_to_series!(i64, objs, name),
        DataType::Float64 => {
            let v = objs
                .iter()
                .map(|&o| lean_unbox_float(o))
                .collect::<Vec<f64>>();
            Series::from_vec(name, v)
        }
    };
    series.to_lean()
}

/// # Safety
/// The lean function must pass a valid series
#[no_mangle]
pub unsafe extern "C" fn polars_lean_print_series(series: b_lean_obj_arg) -> lean_obj_res {
    let series: *mut Series = lean_get_external_data(series);
    let s = series.as_ref().unwrap().to_string();
    convert_string(s.as_str())
}
