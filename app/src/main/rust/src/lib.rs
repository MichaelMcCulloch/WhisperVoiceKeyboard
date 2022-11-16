use std::ptr::NonNull;

use jni::{
    objects::{JClass, JString},
    sys::{jobject, jstring},
    JNIEnv,
};
use ndk::asset::AssetManager;

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
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_retrieveAsset(
    env: JNIEnv,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _class: JClass,
    assetManager: jobject,
) -> jstring {
    let manager = unsafe {
        let ptr = NonNull::new_unchecked(assetManager as *mut _);
        AssetManager::from_ptr(ptr)
    };
    let output = env
        .new_string(format!("Success!"))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}
