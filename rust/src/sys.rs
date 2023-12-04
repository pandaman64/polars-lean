#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::useless_transmute)]

use core::{ffi::c_void, mem::size_of};
use std::ffi::{c_int, c_uint};

#[repr(transparent)]
pub(crate) struct SyncPtr<T>(pub(crate) *mut T);

unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// static inline functions
#[inline]
pub(crate) fn lean_is_scalar(o: *mut lean_object) -> bool {
    (o as usize) & 1 == 1
}

#[inline]
pub(crate) fn lean_box(n: usize) -> *mut lean_object {
    ((n << 1) | 1) as *mut lean_object
}

#[inline]
pub(crate) fn lean_unbox(o: *mut lean_object) -> usize {
    (o as usize) >> 1
}

#[inline]
fn lean_align(size: u32, alignment: u32) -> u32 {
    (size / alignment) * alignment + if size % alignment != 0 { alignment } else { 0 }
}

#[inline]
fn lean_get_slot_idx(sz: u32) -> u32 {
    debug_assert!(sz > 0);
    debug_assert_eq!(lean_align(sz, LEAN_OBJECT_SIZE_DELTA), sz);
    sz / LEAN_OBJECT_SIZE_DELTA - 1
}

#[inline]
unsafe fn lean_set_st_header(o: *mut lean_object, tag: u32, other: u32) {
    (*o).m_rc = 1;
    (*o).set_m_tag(tag);
    (*o).set_m_other(other);
    (*o).set_m_cs_sz(0);
}

#[inline]
pub(crate) unsafe fn lean_alloc_small_object(sz: u32) -> *mut lean_object {
    // assumes LEAN_SMALL_ALLOCATOR
    let sz = lean_align(sz, LEAN_OBJECT_SIZE_DELTA);
    let slot_idx = lean_get_slot_idx(sz);
    debug_assert!(sz <= LEAN_MAX_SMALL_OBJECT_SIZE);
    lean_alloc_small(sz, slot_idx) as _
}

#[inline]
pub(crate) unsafe fn lean_alloc_external(
    cls: *mut lean_external_class,
    data: *mut c_void,
) -> *mut lean_object {
    let o: *mut lean_external_object =
        lean_alloc_small_object(size_of::<lean_external_object>() as u32).cast();
    lean_set_st_header(o.cast(), LeanExternal, 0);
    (*o).m_class = cls;
    (*o).m_data = data;
    o.cast()
}

#[inline]
pub(crate) unsafe fn lean_get_external_data<T>(o: *mut lean_object) -> *mut T {
    debug_assert!((*o).m_tag() == LeanExternal);
    (*o.cast::<lean_external_object>()).m_data.cast()
}

#[inline]
pub(crate) unsafe fn lean_alloc_ctor_memory(sz: u32) -> *mut lean_object {
    // assumes LEAN_SMALL_ALLOCATOR
    let sz1 = lean_align(sz, LEAN_OBJECT_SIZE_DELTA);
    let slot_idx = lean_get_slot_idx(sz1);
    debug_assert!(sz1 <= LEAN_MAX_SMALL_OBJECT_SIZE);
    let r: *mut _ = lean_alloc_small(sz1, slot_idx);
    if sz1 > sz {
        r.cast::<u8>()
            .offset(sz1 as isize)
            .cast::<usize>()
            .offset(-1)
            .write(0);
    }
    r.cast()
}

#[inline]
pub(crate) unsafe fn lean_alloc_ctor(tag: u32, num_objs: u32, scalar_sz: u32) -> *mut lean_object {
    debug_assert!(
        tag <= LeanMaxCtorTag
            && num_objs < LEAN_MAX_CTOR_FIELDS
            && scalar_sz < LEAN_MAX_CTOR_SCALARS_SIZE
    );
    let sz = size_of::<lean_ctor_object>() as u32
        + size_of::<*mut std::ffi::c_void>() as u32 * num_objs
        + scalar_sz;
    let o = lean_alloc_ctor_memory(sz);
    lean_set_st_header(o, tag, num_objs);
    o
}

#[inline]
pub(crate) unsafe fn lean_ctor_num_objs(o: *mut lean_object) -> u32 {
    debug_assert!((*o).m_tag() <= LeanMaxCtorTag);
    (*o).m_other()
}

#[inline]
pub(crate) unsafe fn lean_ctor_obj_cptr(o: *mut lean_object) -> *mut *mut lean_object {
    debug_assert!((*o).m_tag() <= LeanMaxCtorTag);
    (*o.cast::<lean_ctor_object>()).m_objs.as_mut_ptr()
}

#[inline]
pub(crate) unsafe fn lean_ctor_get(o: b_lean_obj_arg, i: u32) -> *mut lean_object {
    debug_assert!(i < lean_ctor_num_objs(o));
    lean_ctor_obj_cptr(o).add(i as usize).read()
}

#[inline]
pub(crate) unsafe fn lean_ctor_set(o: b_lean_obj_arg, i: u32, v: lean_obj_arg) {
    debug_assert!(i < lean_ctor_num_objs(o));
    lean_ctor_obj_cptr(o).add(i as usize).write(v);
}

#[inline]
pub(crate) unsafe fn lean_ctor_get_uint32(o: b_lean_obj_arg, offset: u32) -> u32 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * size_of::<*mut lean_object>() as u32);
    lean_ctor_obj_cptr(o).cast::<u8>().add(offset as usize).cast::<u32>().read()
}

#[inline]
pub(crate) unsafe fn lean_ctor_get_uint64(o: b_lean_obj_arg, offset: u32) -> u64 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * size_of::<*mut lean_object>() as u32);
    lean_ctor_obj_cptr(o).cast::<u8>().add(offset as usize).cast::<u64>().read()
}

#[inline]
pub(crate) unsafe fn lean_ctor_get_float(o: b_lean_obj_arg, offset: u32) -> f64 {
    debug_assert!(offset >= lean_ctor_num_objs(o) * size_of::<*mut lean_object>() as u32);
    lean_ctor_obj_cptr(o).cast::<u8>().add(offset as usize).cast::<f64>().read()
}

#[inline]
pub(crate) unsafe fn lean_ctor_set_uint32(o: b_lean_obj_arg, offset: u32, v: u32) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * size_of::<*mut lean_object>() as u32);
    lean_ctor_obj_cptr(o).cast::<u8>().add(offset as usize).cast::<u32>().write(v);
}

#[inline]
pub(crate) unsafe fn lean_ctor_set_uint64(o: b_lean_obj_arg, offset: u32, v: u64) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * size_of::<*mut lean_object>() as u32);
    lean_ctor_obj_cptr(o).cast::<u8>().add(offset as usize).cast::<u64>().write(v);
}

#[inline]
pub(crate) unsafe fn lean_ctor_set_float(o: b_lean_obj_arg, offset: u32, v: f64) {
    debug_assert!(offset >= lean_ctor_num_objs(o) * size_of::<*mut lean_object>() as u32);
    lean_ctor_obj_cptr(o).cast::<u8>().add(offset as usize).cast::<f64>().write(v);
}

#[inline]
pub(crate) unsafe fn lean_box_u32(v: u32) -> lean_obj_res {
    if size_of::<*mut std::ffi::c_void>() == 4 {
        let r = lean_alloc_ctor(0, 0, size_of::<u32>() as u32);
        lean_ctor_set_uint32(r, 0, v);
        r
    } else {
        lean_box(v as usize)
    }
}

#[inline]
pub(crate) unsafe fn lean_unbox_uint32(o: b_lean_obj_arg) -> u32 {
    if size_of::<*mut std::ffi::c_void>() == 4 {
        lean_ctor_get_uint32(o, 0)
    } else {
        lean_unbox(o) as u32
    }
}

#[inline]
pub(crate) unsafe fn lean_box_uint64(v: u64) -> lean_obj_res {
    let r = lean_alloc_ctor(0, 0, size_of::<u64>() as u32);
    lean_ctor_set_uint64(r, 0, v);
    r
}

#[inline]
pub(crate) unsafe fn lean_unbox_uint64(o: b_lean_obj_arg) -> u64 {
    lean_ctor_get_uint64(o, 0)
}

#[inline]
pub(crate) unsafe fn lean_box_float(v: f64) -> lean_obj_res {
    let r = lean_alloc_ctor(0, 0, size_of::<f64>() as u32);
    lean_ctor_set_float(r, 0, v);
    r
}

#[inline]
pub(crate) unsafe fn lean_unbox_float(o: b_lean_obj_arg) -> f64 {
    lean_ctor_get_float(o, 0)
}

// Assumption: integers in [LEAN_MIN_SMALL_INT, LEAN_MAX_SMALL_INT] are encoded by boxing.
// Otherwise, we use bigint.
const LEAM_MAX_SMALL_INT: c_int = if size_of::<*mut c_void>() == 8 {
    c_int::MAX
} else {
    c_int::MAX >> 1
};
const LEAN_MIN_SMALL_INT: c_int = if size_of::<*mut c_void>() == 8 {
    c_int::MIN
} else {
    c_int::MIN >> 1
};

#[inline]
pub(crate) unsafe fn lean_int64_to_int(n: i64) -> lean_obj_res {
    if LEAN_MIN_SMALL_INT as i64 <= n && n <= LEAN_MIN_SMALL_INT as i64 {
        lean_box(n as c_int as c_uint as usize)
    } else {
        lean_big_int64_to_int(n)
    }
}

#[inline]
pub(crate) unsafe fn lean_scalar_to_int64(o: b_lean_obj_arg) -> i64 {
    debug_assert!(lean_is_scalar(o));
    if size_of::<*mut c_void>() == 8 {
        lean_unbox(o) as c_uint as c_int as i64
    } else {
        (o as usize as c_int >> 1) as i64
    }
}

// Utility function not in lean.h
pub(crate) unsafe fn convert_string(s: &str) -> lean_obj_res {
    lean_mk_string_from_bytes(s.as_ptr().cast(), s.len())
}
