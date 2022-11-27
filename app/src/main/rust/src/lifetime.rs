use std::ffi::c_void;

use android_logger::Config;
use jni::{objects::JObject, JNIEnv};
use jni_sys::JavaVM;
use log::Level;

pub(crate) fn init(env: JNIEnv, mut context: JObject) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    unsafe {
        let vm: *mut JavaVM = env.get_java_vm().unwrap().get_java_vm_pointer();
        ndk_context::initialize_android_context(
            vm as *mut _ as *mut c_void,
            &mut context as *mut _ as *mut c_void,
        );
    }
    log::info!("Succeeded in init context")
}

pub(crate) fn uninit() {
    unsafe {
        ndk_context::release_android_context();
        // uninit_audio();
    }
    log::info!("Succeeded in deinit context")
}
