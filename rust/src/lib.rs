use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use anyhow::anyhow;
use anyhow::Result;
use jni::sys::{jint, jsize};
use jni::JavaVM;
use logger::JavaLogger;

mod keystore;
mod logger;

pub fn rust_greeting(to: *const c_char) -> Result<*mut c_char> {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    let vm = get_java_vm()?;
    let mut env = vm.get_env()?;
    let mut logger = JavaLogger::new(&mut env, "RustGreetings").unwrap();

    let mut env = vm.get_env()?;
    logger
        .debug(&mut env, "Hello ".to_owned() + recipient)
        .expect("Unable to log message");

    Ok(CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw())
}

fn get_java_vm<'a>() -> anyhow::Result<JavaVM> {
    // using jni_sys::JNI_GetCreatedJavaVMs crashes, bc the symbol is not loaded into the process for some reason
    // instead we use libloading to load the symbol ourselves
    pub type JniGetCreatedJavaVms = unsafe extern "system" fn(
        vmBuf: *mut *mut jni::sys::JavaVM,
        bufLen: jsize,
        nVMs: *mut jsize,
    ) -> jint;
    pub const JNI_GET_JAVA_VMS_NAME: &[u8] = b"JNI_GetCreatedJavaVMs";

    let lib = libloading::os::unix::Library::this();
    let get_created_java_vms: JniGetCreatedJavaVms =
        unsafe { *lib.get(JNI_GET_JAVA_VMS_NAME).unwrap() };

    // now that we have the function, we can call it
    let mut buffer = [0 as *mut jni::sys::JavaVM; 1];
    let buffer_ptr = buffer.as_mut_ptr();
    let mut found_vms = 0;
    let found_vm_ptr = &mut found_vms as *mut i32;
    let res = unsafe { get_created_java_vms(buffer_ptr, 1, found_vm_ptr) };

    if res != jni::sys::JNI_OK {
        return Err(anyhow!("Unable to get created JVMs"));
    }

    if found_vms == 0 {
        return Err(anyhow!("No JVMs found"));
    }

    let jvm = unsafe { JavaVM::from_raw(buffer[0])? };
    jvm.attach_current_thread()?;
    Ok(jvm)
}

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_greetings_RustGreetings_greeting(
        mut env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.

        let world = match rust_greeting(
            env.get_string(&java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        ) {
            Ok(res) => res,
            Err(_) => {
                CString::new("Error in rust_greeting").unwrap().into_raw()
            }
        };
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let world_ptr = CString::from_raw(world);
        let output = env
            .new_string(world_ptr.to_str().unwrap())
            .expect("Couldn't create java string!");

        **output
    }
}
