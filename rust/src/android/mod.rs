// #![cfg(target_os = "android")]
#![allow(non_snake_case)]


use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use super::*;

/// get the greeting from the rust side
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_greeting(
    mut env: JNIEnv,
    _: JClass,
    java_pattern: JString,
) -> jstring {
    // Our Java companion code might pass-in "world" as a string, hence the name.

    let world = rust_greeting(
        env.get_string(&java_pattern)
            .expect("invalid pattern string")
            .as_ptr(),
    )
    .unwrap_or(CString::new("Error in rust_greeting").unwrap());
    // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
    let output = env
        .new_string(world.to_str().unwrap())
        .expect("Couldn't create java string!");

    **output
}