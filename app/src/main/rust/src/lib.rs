use std::any::Any;
use std::ffi::{c_void, CString};
use std::ptr::NonNull;

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
    let aasset_manager_pointer = unsafe {
        ndk_sys::AAssetManager_fromJava(env.get_native_interface(), *asset_manager_object)
    };
    let asset_manager = unsafe {
        ndk::asset::AssetManager::from_ptr(NonNull::<ndk_sys::AAssetManager>::new_unchecked(
            aasset_manager_pointer,
        ))
    };
    log::info!("{:?}", asset_manager.type_id());
    match CString::new("whisper.tflite") {
        Ok(cstring) => match asset_manager.open(&cstring) {
            Some(mut asset) => {
                log::info!("Success opening asset!");
                match asset.get_buffer() {
                    Ok(b) => {
                        log::info!("Success Loading Model!")
                    }
                    Err(_) => {}
                }
            }
            None => log::warn!("Fail Open"),
        },
        Err(_) => log::info!("Fail String"),
    };
}
