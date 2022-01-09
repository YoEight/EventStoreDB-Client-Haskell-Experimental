use std::{ffi::CStr, os::raw::c_char};
use tokio::runtime::Runtime;

extern crate libc;

pub struct ES_Env {
    runtime: *mut Runtime,
    client: *mut eventstore::Client,
}

#[repr(C)]
pub struct EventData {
    event_type: *const c_char,
    payload: *const [u8],
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

#[no_mangle]
pub extern "C" fn append_to_stream(
    env: *mut ES_Env,
    stream_name: *const c_char,
    event_data: *const EventData,
) {
    let event_type = unsafe { CStr::from_ptr((*event_data).event_type) };
    let stream_name = unsafe { CStr::from_ptr(stream_name) };
    let event_type = event_type.to_str().expect("Invalid UTF-8 string");
    let stream_name = stream_name.to_str().expect("Invalid UTF-8 string");
    let payload = unsafe { (*event_data).payload.as_ref().expect("payload was null") };

    let data = eventstore::EventData::binary(event_type, payload.into());
    let runtime = unsafe { (*env).runtime.as_ref().expect("runtime must be defined") };
    let client = unsafe { (*env).client.as_ref().expect("client must be defined") };
    let handle = runtime.handle().clone();
    handle
        .block_on(client.append_to_stream(stream_name, &Default::default(), data))
        .unwrap();
}
