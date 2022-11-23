use std::any::Any;
use std::ffi::{c_void, CString};
use std::fs::File;
use std::panic::catch_unwind;
use std::ptr::NonNull;

mod asset_helper;

use jni_sys::JNI_VERSION_1_8;
use tflitec::interpreter::{Interpreter, Options};

use crate::asset_helper::load_asset_manager;

const WHISPER_TFLITE: &str = "whisper.tflite";

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_hello(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    input: jni::objects::JString,
) -> jni::sys::jstring {
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
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_init(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    mut context: jni::objects::JObject,
) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    unsafe {
        let vm: *mut jni_sys::JavaVM = env.get_java_vm().unwrap().get_java_vm_pointer();
        ndk_context::initialize_android_context(
            vm as *mut _ as *mut c_void,
            &mut context as *mut _ as *mut c_void,
        );
    }

    log::info!("Succeeded in init context")
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_retrieveAsset(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    asset_manager_object: jni::objects::JObject,
) {
    let asset_manager = load_asset_manager(env, asset_manager_object);

    let mut tflite_file = asset_helper::load_asset_buffer(WHISPER_TFLITE, &asset_manager)
        .expect(format!("Could not load {}", WHISPER_TFLITE).as_str());
    let tflite_buf = tflite_file
        .get_buffer()
        .expect("File opened, but no data read from buffer!");

    let _interpreter = Interpreter::with_model_bytes(
        tflite_buf,
        tflite_buf.len() as u64,
        Some(Options::default()),
    )
    .expect(
        format!(
            "Could not create a TfLiteC-rs Interpreter with a collection of bits named {}",
            WHISPER_TFLITE
        )
        .as_str(),
    );

    log::info!("Success Loading Model!");
}
