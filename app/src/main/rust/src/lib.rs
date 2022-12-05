use std::{
    ffi::c_void,
    mem::{self, ManuallyDrop},
    str::FromStr,
    usize,
};

use ac_ffmpeg::codec::audio::{
    resampler::{AudioResampler, AudioResamplerBuilder},
    ChannelLayout, SampleFormat,
};
use android_logger::Config;
use jni::{
    objects::{JByteBuffer, JClass, JObject},
    sys::{jboolean, jint},
    JNIEnv,
};
use log::Level;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_initLogger(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_example_whisperVoiceRecognition_RustLib_createLogMelSpectogramFromAudioBytes(
    env: JNIEnv,
    _class: JClass,
    audio_buffer: JByteBuffer,
    output_buffer: JByteBuffer,
) -> jboolean {
    match create_log_mel_spectrogram_from_audio_bytes(env, audio_buffer, output_buffer) {
        Ok(_) => true.into(),
        Err(_) => false.into(),
    }
}

fn create_log_mel_spectrogram_from_audio_bytes(
    env: JNIEnv,
    audio_buffer: JByteBuffer,
    output_buffer: JByteBuffer,
) -> anyhow::Result<()> {
    // let bytes = read_jbyte_buffer(env, audio_buffer)?;
    // let mut output = read_jbyte_buffer(env, output_buffer)?;
    let resampler = AudioResampler::builder()
        .source_channel_layout(ChannelLayout::from_channels(2).unwrap())
        .source_sample_format(SampleFormat::from_str("AV_SAMPLE_FMT_S16").unwrap())
        .source_sample_rate(48000)
        .target_channel_layout(ChannelLayout::from_channels(2).unwrap())
        .target_sample_format(SampleFormat::from_str("AV_SAMPLE_FMT_S16").unwrap())
        .target_sample_rate(48000)
        .target_frame_samples(Some(2))
        .build()
        .unwrap();

    // let floats = bytes
    //     .chunks_exact(4)
    //     .map(|four_bytes| {
    //         let u = [four_bytes[0], four_bytes[1], four_bytes[2], four_bytes[3]];
    //         let f32 = f32::from_be_bytes(u);
    //         f32
    //     })
    //     .collect::<Vec<_>>();
    // let float_write = floats.clone();
    // let bytes_write = float_write
    //     .into_iter()
    //     .flat_map(|f| f.to_be_bytes())
    //     .collect::<Vec<_>>();
    // output.copy_from_slice(&bytes_write);
    Ok(())
}

fn read_jbyte_buffer(
    env: JNIEnv,
    audio_buffer: JByteBuffer,
) -> anyhow::Result<ManuallyDrop<Vec<u8>>> {
    let buffer_address = env.get_direct_buffer_address(audio_buffer)?;
    let buffer_cap = env.get_direct_buffer_capacity(audio_buffer)?;
    let bytes = unsafe { Vec::from_raw_parts(buffer_address, buffer_cap, buffer_cap) };
    let bytes = ManuallyDrop::new(bytes);
    Ok(bytes)
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
///
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
