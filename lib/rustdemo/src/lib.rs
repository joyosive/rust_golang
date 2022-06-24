extern crate libc;
use std::ffi::{CStr, CString};


// function name exposed by rust is called rustdemo,
// receive an external parameter and print it out.
// Then set a string from rust.

#[no_mangle]
pub extern "C" fn rustdemo(name: *const libc::c_char) -> *const libc::c_char {
    let cstr_name = unsafe { CStr::from_ptr(name) };
    let mut str_name = cstr_name.to_str().unwrap().to_string();
    println!("Rust get Input:  \"{}\"", str_name);
    let r_string: &str = " Rust 2 Go ";
    str_name.push_str(r_string);
    CString::new(str_name).unwrap().into_raw()
}
