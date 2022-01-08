use std::{ffi::CStr, os::raw::c_char};
use tokio::runtime::Runtime;

extern crate libc;

pub struct ES_Env {
    runtime: *mut Runtime,
    client: *mut eventstore::Client,
}

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

#[no_mangle]
pub extern "C" fn create_es_env(conn_string: *const c_char) -> *mut ES_Env {
    unsafe {
        let conn_string = CStr::from_ptr(conn_string);
        if let Ok(conn_string) = conn_string.to_str() {
            if let Ok(setts) = conn_string.parse() {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .expect("Can't create a runtime");

                let handle = runtime.handle().clone();
                let client = eventstore::Client::with_runtime_handle(handle, setts)
                    .expect("Invalid environment");
                let client = Box::into_raw(Box::new(client));
                let runtime = Box::into_raw(Box::new(runtime));

                return Box::into_raw(Box::new(ES_Env { runtime, client }));
            } else {
                panic!("Wrong connection setting format!");
            }
        } else {
            panic!("Non UTF-8 string");
        }
    }
}
