#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::useless_transmute)]

use core::ffi::c_void;

#[repr(transparent)]
pub(crate) struct SyncPtr<T>(pub(crate) *mut T);

unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// static inline functions
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
        lean_alloc_small_object(core::mem::size_of::<lean_external_object>() as u32).cast();
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

pub(crate) unsafe fn convert_string(s: &str) -> lean_obj_res {
    lean_mk_string_from_bytes(s.as_ptr().cast(), s.len())
}
