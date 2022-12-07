use jni::{
    sys::{jboolean, jint, jobject},
    JNIEnv,
};

mod jni_util;
mod job;
mod lifetime;
mod record;
mod statics;

pub(crate) enum Message {
    Stop,
    Abort,
    Pause,
    Resume,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_init(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) {
    lifetime::init();
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_uninit(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) {
    lifetime::uninit();
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_startRecording(
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
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_endRecording(
    env: JNIEnv,
    _class: jni::objects::JClass,
) -> jobject {
    match record::request_end() {
        Some(mut data) => unsafe { env.new_direct_byte_buffer(data.as_mut_ptr(), data.len()) },
        None => unsafe { env.new_direct_byte_buffer(vec![].as_mut_ptr(), 0) },
    }
    .unwrap()
    .into_raw()
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_abortRecording(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) -> jboolean {
    record::request_abort().into()
}
