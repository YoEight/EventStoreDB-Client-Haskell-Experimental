use std::{ffi::CStr, os::raw::c_char};

extern crate libc;

#[no_mangle]
pub extern "C" fn print_string(x: *const c_char) {
    unsafe {
        let cstring = CStr::from_ptr(x);

        if let Ok(input) = cstring.to_str() {
            println!("Message from Haskell: {}", input);
        } else {
            panic!("Non UTF-8 string");
        }
    }
}
