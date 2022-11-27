use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

use anyhow::Result;
use jni::{
    sys::{jboolean, jint, jstring},
    JNIEnv,
};

static mut VOICE_PROCESSING_THREAD: Option<JoinHandle<Option<String>>> = None;
static mut VOICE_PROCESSING_THREAD_MESSENGER: Option<Sender<Message>> = None;

enum Message {
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

    // let input_stream = AudioStreamBuilder::new()
    //     .expect("Could not get Audio Stream Builder")
    //     .device_id(audio_device_id)
    //     .direction(AudioDirection::Input)
    //     .sharing_mode(AudioSharingMode::Shared)
    //     .performance_mode(ndk::audio::AudioPerformanceMode::LowLatency)
    //     .sample_rate(audio_device_sample_rate)
    //     .channel_count(audio_device_channel_count)
    //     .format(ndk::audio::AudioFormat::PCM_Float)
    //     .data_callback(Box::new(|a, b, c| -> AudioCallbackResult {
    //         //let float_arr = b as *mut c_float;
    //         //log::info!("{:?}", c);
    //         //unsafe {
    //         //    let vec = slice::from_raw_parts(float_arr, c as usize).to_vec();
    //         //}
    //         AudioCallbackResult::Continue
    //     }))
    //     .open_stream()
    //     .expect("Could not get AudioStream");
    match message_receiver.recv() {
        Ok(Message::StopAndTranscribe) => Some(String::from("Hellop there")),
        Ok(Message::Abort) => None,
        Err(_) => None,
    }
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

pub(crate) fn abort_recording() -> jboolean {
    match unsafe {
        (
            VOICE_PROCESSING_THREAD.take(),
            VOICE_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(join_handle), Some(messenger)) => {
            messenger
                .send(Message::Abort)
                .expect("Could not deliver Message::Abort");
            join_handle.join().expect("Thread is dead?");
            true.into()
        }
        (_, _) => false.into(),
    }
}
