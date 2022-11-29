use anyhow::Result;
use crossbeam_queue::ArrayQueue;
use jni::{
    sys::{jboolean, jint, jstring},
    JNIEnv,
};
use ndk::audio::{
    AudioAllowedCapturePolicy, AudioCallbackResult, AudioDirection, AudioPerformanceMode,
    AudioSharingMode, AudioStream, AudioStreamBuilder, AudioStreamState,
};
use std::{
    slice,
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::statics::{VOICE_PROCESSING_THREAD, VOICE_PROCESSING_THREAD_MESSENGER};

pub(crate) enum Message {
    StopAndTranscribe,
    Abort,
}

fn transcription_thread(
    audio_device_id: i32,
    audio_device_sample_rate: i32,
    audio_device_channel_count: i32,
    message_receiver: Receiver<Message>,
) -> Option<String> {
    // Spin up audio stream and prepare callback
    let intervals_per_second = 100i32;
    let thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>> =
        Arc::new(ArrayQueue::new((intervals_per_second * 30) as usize));

    let input_stream = create_audio_stream(
        audio_device_channel_count,
        audio_device_sample_rate,
        intervals_per_second,
        thirty_second_audio_buffer.clone(),
        audio_device_id,
    );

    input_stream.request_start().unwrap();

    match message_receiver.recv() {
        message => {
            input_stream.request_stop().unwrap();
            input_stream
                .wait_for_state_change(
                    AudioStreamState::Stopping,
                    Duration::from_secs(1).as_nanos() as i64,
                )
                .unwrap();

            match message {
                Ok(Message::StopAndTranscribe) => {
                    let capacity =
                        (audio_device_sample_rate * audio_device_channel_count * 30) as usize;
                    let audio_buffer = {
                        let mut audio_buffer = Vec::with_capacity(capacity);

                        while let Some(mut vec_frames) = thirty_second_audio_buffer.pop() {
                            audio_buffer.append(&mut vec_frames)
                        }

                        let mut pad_the_buffer_with_zeroes =
                            vec![0i16; capacity - audio_buffer.len()];
                        audio_buffer.append(&mut pad_the_buffer_with_zeroes);
                        log::info!("{:?}", audio_buffer.len());
                        audio_buffer
                    };
                    //convert the input signal to 16khz mono

                    let audio_buffer_mono = if audio_device_channel_count == 2 {
                        combine_channels(audio_buffer)
                    } else {
                        audio_buffer
                    };

                    Some(String::from("Hello there!"))
                }
                Ok(Message::Abort) => None,
                Err(_) => None,
            }
        }
    }
}

fn combine_channels(stereo_pairwise: Vec<i16>) -> Vec<i16> {
    stereo_pairwise.chunks_exact(2).into_iter().fold(
        Vec::with_capacity(stereo_pairwise.len() / 2),
        |mut mono, s| {
            mono.push(((s[0] as i32 + s[1] as i32) / 2) as i16);
            mono
        },
    )
}

fn create_audio_stream(
    audio_device_channel_count: i32,
    audio_device_sample_rate: i32,
    intervals_per_second: i32,
    thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>>,
    audio_device_id: i32,
) -> AudioStream {
    let samples_per_interval =
        audio_device_channel_count * audio_device_sample_rate / intervals_per_second;
    let input_stream = AudioStreamBuilder::new()
        .expect("Could not get Audio Stream Builder")
        .device_id(audio_device_id)
        .direction(AudioDirection::Input)
        .sharing_mode(AudioSharingMode::Shared)
        .performance_mode(AudioPerformanceMode::LowLatency)
        .frames_per_data_callback(samples_per_interval)
        .sample_rate(audio_device_sample_rate)
        .channel_count(audio_device_channel_count)
        .format(ndk::audio::AudioFormat::PCM_I16)
        .allowed_capture_policy(AudioAllowedCapturePolicy::AllowCaptureByNone)
        .data_callback(Box::new(
            move |_audio_stream, frame_buffer, count| -> AudioCallbackResult {
                let i16_array = frame_buffer as *mut i16; //TODO try i32 or c_int if this doesn't work!
                let vec_audio_frames =
                    unsafe { slice::from_raw_parts(i16_array, count as usize).to_vec() };

                match thirty_second_audio_buffer.push(vec_audio_frames) {
                    Ok(()) => AudioCallbackResult::Continue,
                    Err(_) => AudioCallbackResult::Stop,
                }
            },
        ))
        .open_stream()
        .expect("Could not get AudioStream");
    log_trace_audio_stream_info(&input_stream);
    input_stream
}

fn log_trace_audio_stream_info(input_stream: &AudioStream) {
    log::trace!("get_format:\t\t\t\t\t\t\t\t{:?}", input_stream.get_format());
    log::trace!(
        "get_sample_rate:\t\t\t\t\t\t{}",
        input_stream.get_sample_rate()
    );
    log::trace!(
        "get_samples_per_frame:\t\t\t\t\t{}",
        input_stream.get_samples_per_frame()
    );
    log::trace!(
        "get_frames_per_burst:\t\t\t\t\t{}",
        input_stream.get_frames_per_burst()
    );
    log::trace!(
        "get_channel_count:\t\t\t\t\t\t{}",
        input_stream.get_channel_count()
    );
    log::trace!(
        "get_buffer_capacity_in_frames:\t\t\t{}",
        input_stream.get_buffer_capacity_in_frames()
    );
    log::trace!(
        "get_buffer_size_in_frames:\t\t\t\t{}",
        input_stream.get_buffer_size_in_frames()
    );
    log::trace!(
        "get_frames_per_data_callback:\t\t\t{:?}",
        input_stream.get_frames_per_data_callback()
    );
}

pub(crate) fn start_recording(device_id: jint, sample_rate: jint, channels: jint) -> jboolean {
    match unsafe {
        (
            VOICE_PROCESSING_THREAD.take(),
            VOICE_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (None, None) => {
            let (sender, recv) = channel();
            let join_handle =
                thread::spawn(move || transcription_thread(device_id, sample_rate, channels, recv));

            unsafe { VOICE_PROCESSING_THREAD.replace(join_handle) };
            unsafe { VOICE_PROCESSING_THREAD_MESSENGER.replace(sender) };

            true.into()
        }
        (_, _) => false.into(),
    }
}

pub(crate) fn end_recording(env: JNIEnv) -> Result<jstring> {
    Ok(
        match unsafe {
            (
                VOICE_PROCESSING_THREAD.take(),
                VOICE_PROCESSING_THREAD_MESSENGER.take(),
            )
        } {
            (Some(join_handle), Some(messenger)) => {
                messenger.send(Message::StopAndTranscribe)?;
                let transcription = match join_handle.join() {
                    Ok(Some(string)) => string,
                    Ok(None) => String::new(),
                    Err(_) => String::new(),
                };
                let output = env.new_string(transcription)?;
                output.into_raw()
            }
            (_, _) => {
                let output = env.new_string(format!("*throws an error in shame*"))?;
                output.into_raw()
            }
        },
    )
}

pub(crate) fn abort_recording() -> Result<jboolean> {
    Ok(
        match unsafe {
            (
                VOICE_PROCESSING_THREAD.take(),
                VOICE_PROCESSING_THREAD_MESSENGER.take(),
            )
        } {
            (Some(join_handle), Some(messenger)) => {
                messenger.send(Message::Abort)?;
                join_handle.join().unwrap();
                true.into()
            }
            (_, _) => false.into(),
        },
    )
}
