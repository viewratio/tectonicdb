extern crate libc;
use self::libc::c_char;
use std::ffi::{CStr, CString};
use std::mem;

use dtf::{self, Update};

#[repr(C)]
pub struct Slice {
    ptr: *mut Update,
    len: usize,
}

#[no_mangle]
pub extern fn read_dtf_to_csv(fname: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!fname.is_null());
        CStr::from_ptr(fname)
    };
    let fname = c_str.to_str().unwrap();

    let ups = dtf::decode(fname, None).unwrap();
    let data = dtf::update_vec_to_csv(&ups);

    let ret = String::from(data);
    let c_str_song = CString::new(ret).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern fn read_dtf_to_csv_with_limit(fname: *const c_char, num: u32) -> *mut c_char {
    let c_str = unsafe {
        assert!(!fname.is_null());
        CStr::from_ptr(fname)
    };
    let fname = c_str.to_str().unwrap();

    let ups = dtf::decode(fname, Some(num)).unwrap();
    let data = dtf::update_vec_to_csv(&ups);

    let ret = String::from(data);
    let c_str_song = CString::new(ret).unwrap();
    c_str_song.into_raw()
}

#[no_mangle]
pub extern fn read_dtf_to_arr(fname: *const c_char) -> Slice {
    let c_str = unsafe {
        assert!(!fname.is_null());
        CStr::from_ptr(fname)
    };
    let fname = c_str.to_str().unwrap();

    let mut ups = dtf::decode(fname, None).unwrap();

    let p = ups.as_mut_ptr();
    let len = ups.len();

    // so that no destructor is run on our vector
    mem::forget(ups);

    Slice { ptr: p, len: len }
}

#[no_mangle]
pub extern fn read_dtf_to_arr_with_limit(fname: *const c_char, num: u32) -> Slice {
    let c_str = unsafe {
        assert!(!fname.is_null());
        CStr::from_ptr(fname)
    };
    let fname = c_str.to_str().unwrap();

    let mut ups = dtf::decode(fname, Some(num)).unwrap();

    let p = ups.as_mut_ptr();
    let len = ups.len();

    // so that no destructor is run on our vector
    mem::forget(ups);

    Slice { ptr: p, len: len }
}

#[no_mangle]
pub extern fn str_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}