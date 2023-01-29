use std::mem::ManuallyDrop;

use jni::{
    objects::JObject,
    sys::{jboolean, jint, jobject},
    JNIEnv,
};

mod asset;
mod consts;
mod extract;
mod job;
mod lifetime;
mod mel;
mod record;
mod spectrogram;
mod statics;
mod work_buffer;
pub(crate) enum Message {
    Stop,
    Abort,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperVoiceRecognition_RustLib_init(
    env: JNIEnv,
    _class: jni::objects::JClass,
    asset_manager: JObject,
) {
    let filters_and_vocab_bits = asset::obtain_filters_vocab_binary_data(env, asset_manager)
        .expect("Could not obtain Filters, Crashing. Obviously.");
    lifetime::init(filters_and_vocab_bits);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperVoiceRecognition_RustLib_uninit(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) {
    lifetime::uninit();
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperVoiceRecognition_RustLib_startRecording(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    device_id: jint,
    sample_rate_hz: jint,
    channel_count: jint,
) -> jboolean {
    record::request_start(device_id, sample_rate_hz, channel_count).into()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperVoiceRecognition_RustLib_endRecording(
    env: JNIEnv,
    _class: jni::objects::JClass,
) -> jobject {
    match record::request_end() {
        Some(data) => unsafe {
            env.new_direct_byte_buffer(ManuallyDrop::new(data).as_mut_ptr(), 960000)
        },
        None => unsafe { env.new_direct_byte_buffer(ManuallyDrop::new(vec![]).as_mut_ptr(), 0) },
    }
    .unwrap()
    .into_raw()
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperVoiceRecognition_RustLib_abortRec(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) -> jboolean {
    record::request_abort().into()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperkeyboardwatch_RustLib_init(
    env: JNIEnv,
    _class: jni::objects::JClass,
    asset_manager: JObject,
) {
    let filters_and_vocab_bits = asset::obtain_filters_vocab_binary_data(env, asset_manager)
        .expect("Could not obtain Filters, Crashing. Obviously.");
    lifetime::init(filters_and_vocab_bits);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperkeyboardwatch_RustLib_uninit(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) {
    lifetime::uninit();
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperkeyboardwatch_RustLib_startRecording(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    device_id: jint,
    sample_rate_hz: jint,
    channel_count: jint,
) -> jboolean {
    record::request_start(device_id, sample_rate_hz, channel_count).into()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperkeyboardwatch_RustLib_endRecording(
    env: JNIEnv,
    _class: jni::objects::JClass,
) -> jobject {
    match record::request_end() {
        Some(data) => unsafe {
            env.new_direct_byte_buffer(ManuallyDrop::new(data).as_mut_ptr(), 960000)
        },
        None => unsafe { env.new_direct_byte_buffer(ManuallyDrop::new(vec![]).as_mut_ptr(), 0) },
    }
    .unwrap()
    .into_raw()
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_mjm_whisperkeyboardwatch_RustLib_abortRec(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) -> jboolean {
    record::request_abort().into()
}
