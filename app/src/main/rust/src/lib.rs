use std::any::Any;
use std::borrow::BorrowMut;
use std::ffi::{c_void, CStr, CString, NulError};
use std::ptr::NonNull;

use android_logger::Config;

use jni::objects::JString;
use jni::{
    objects::{JClass, JObject, JValue},
    JNIEnv,
};

use jni::sys::{jbyteArray, jint, jlong, jstring};
use jni_sys::JavaVM;
use log::Level;
use ndk::asset::Asset;
use ndk::asset::AssetManager;
use ndk_sys::AAssetManager_fromJava;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_hello(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass,
    input: JString,
) -> jstring {
    // First, we have to get the string out of Java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_initLogger(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass,
    mut context: JObject,
) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));

    unsafe {
        let vm: *mut JavaVM = env.get_java_vm().unwrap().get_java_vm_pointer();
        ndk_context::initialize_android_context(
            vm as *mut _ as *mut c_void,
            &mut context as *mut _ as *mut c_void,
        );
        let ctx = ndk_context::android_context();
        // let vm = unsafe { env.get_java_vm() }.unwrap();
        // let env = vm.attach_current_thread().unwrap();
    }

    log::info!("Succeeded in init context")
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_retrieveAsset(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass,
    assetManager: JObject,
) {
    let manager_ptr = unsafe { AAssetManager_fromJava(env.get_native_interface(), *assetManager) };
    let manager = unsafe {
        AssetManager::from_ptr(NonNull::<ndk_sys::AAssetManager>::new_unchecked(
            manager_ptr,
        ))
    };
    log::info!("{:?}", manager.type_id());
    let output = match CString::new("whisper.tflite") {
        Ok(cstring) => match manager.open(&cstring) {
            Some(asset) => log::info!("Success opening asset!"),
            None => log::warn!("Fail Open"),
        },
        Err(_) => log::info!("Fail String"),
    };
}
