use core::slice;
use std::any::Any;
use std::ffi::{c_float, c_void, CString};
use std::fs::File;
use std::panic::catch_unwind;
use std::ptr::NonNull;
use std::thread::{self, JoinHandle};
use std::time::Duration;

mod asset_helper;
mod audio_device_config;
mod lifetime;

use audio_device_config::{CHANNELS, DEVICE_ID, SAMPLE_RATE};
use jni::objects::{JObject, JValue};
use jni_sys::{jint, JNI_VERSION_1_8};
use ndk::audio::{
    AudioCallbackResult, AudioDirection, AudioSharingMode, AudioStream, AudioStreamBuilder,
};
use tflitec::interpreter::{Interpreter, Options};

use crate::asset_helper::get_asset_manager;

const WHISPER_TFLITE: &str = "whisper.tflite";
const FILTERS_VOCAB_GEN_BIN: &str = "filters_vocab_gen.bin";
const GET_DEVICES_OUTPUTS: jni::sys::jint = 2;
const GET_DEVICES_INPUTS: jni::sys::jint = 1;

static mut VOICE_PROCESS_RESULT: Option<JoinHandle<String>> = None;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_startRecording(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    device_id: jint,
    sample_rate: jint,
    channels: jint,
) -> jni::sys::jboolean {
    if unsafe { VOICE_PROCESS_RESULT.is_none() } {
        let join_handle = thread::spawn(|| String::from("capital F false"));
        let _previous = unsafe { VOICE_PROCESS_RESULT.replace(join_handle) };

        true.into()
    } else {
        false.into()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_endRecording(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
) -> jni::sys::jstring {
    if unsafe { VOICE_PROCESS_RESULT.is_some() } {
        let result = unsafe { VOICE_PROCESS_RESULT.take() }.unwrap();
        let output = env
            .new_string(result.join().expect("did not expect audio thread to panic"))
            .expect("Couldn't create java string!");
        output.into_raw()
    } else {
        let output = env
            .new_string(format!("*throws an error in shame*"))
            .expect("Couldn't create java string!");
        output.into_raw()
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_init(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    context: jni::objects::JObject,
) {
    lifetime::init(env, context);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_uninit(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
) {
    lifetime::uninit();
}

/// ```
/// #[no_mangle]
/// #[allow(non_snake_case)]
/// pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_retrieveAsset(
///     env: jni::JNIEnv,
///     _class: jni::objects::JClass,
///     asset_manager_object: jni::objects::JObject,
/// ) {
///     let asset_manager = get_asset_manager(env, asset_manager_object);

///     let mut tflite_file = asset_helper::load_asset_buffer(WHISPER_TFLITE, &asset_manager)
///         .expect(format!("Could not load {}", WHISPER_TFLITE).as_str());
///     let tflite_buf = tflite_file
///         .get_buffer()
///         .expect("File opened, but no data read from buffer!");

///     let _interpreter = Interpreter::with_model_bytes(
///         tflite_buf,
///         tflite_buf.len() as u64,
///         Some(Options::default()),
///     )
///     .expect(
///         format!(
///             "Could not create a TfLiteC-rs Interpreter with a collection of bits named {}",
///             WHISPER_TFLITE
///         )
///         .as_str(),
///     );

///     log::info!("Success Loading Model!");
/// }

/// #[no_mangle]
/// #[allow(non_snake_case)]
/// pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_sampleAudio(
///     _env: jni::JNIEnv,
///     _class: jni::objects::JClass,
/// ) {
///     sample_audio().expect("Something Went Wrong!!!")
/// }

/// fn sample_audio() -> anyhow::Result<()> {
///     match unsafe { (DEVICE_ID, SAMPLE_RATE, CHANNELS) } {
///         (Some(device_id), Some(sample_rate), Some(channels)) => {
///             let input_stream = AudioStreamBuilder::new()?
///                 .device_id(device_id)
///                 .direction(AudioDirection::Input)
///                 .sharing_mode(AudioSharingMode::Shared)
///                 .performance_mode(ndk::audio::AudioPerformanceMode::LowLatency)
///                 .sample_rate(sample_rate)
///                 .channel_count(channels)
///                 .format(ndk::audio::AudioFormat::PCM_Float)
///                 .data_callback(Box::new(|a, b, c| -> AudioCallbackResult {
///                     /// let float_arr = b as *mut c_float;
///                     /// log::info!("{:?}", c);

///                     /// unsafe {
///                     ///     let vec = slice::from_raw_parts(float_arr, c as usize).to_vec();
///                     /// }
///                     AudioCallbackResult::Continue
///                 }))
///                 .open_stream()?;

///             let sample_rate = input_stream.get_sample_rate();
///             let frames_per_burst = input_stream.get_frames_per_burst();

///             input_stream.request_start()?;
///             thread::sleep(Duration::from_secs(10));
///             Ok(())
///         }

///         (_, _, _) => Err(std::io::Error::new(
///             std::io::ErrorKind::PermissionDenied,
///             "You have not initialized the library!!!",
///         )),
///     }?;
///     Ok(())
/// }
/// ```
pub fn sample() {}
