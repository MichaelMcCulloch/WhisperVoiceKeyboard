use std::{
    mem::ManuallyDrop,
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
    time::Duration,
};

use crossbeam_queue::ArrayQueue;
use jni::objects::JByteBuffer;
use ndk::audio::{
    AudioAllowedCapturePolicy, AudioCallbackResult, AudioDirection, AudioPerformanceMode,
    AudioSharingMode, AudioStream, AudioStreamBuilder, AudioStreamState,
};

use crate::{
    statics::{AUDIO_PROCESSING_THREAD, AUDIO_PROCESSING_THREAD_MESSENGER},
    Message,
};

const INTERVALS_PER_SECOND: i32 = 1;

pub(crate) fn request_start(device_id: i32, sample_rate: i32, channels: i32) -> bool {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (None, None) => {
            let (sender, recv) = channel();
            let join_handle =
                thread::spawn(move || audio_job(device_id, sample_rate, channels, recv));

            unsafe {
                AUDIO_PROCESSING_THREAD.replace(join_handle);
                AUDIO_PROCESSING_THREAD_MESSENGER.replace(sender);
            }
            true
        }
        (_, _) => {
            log::error!("Cannot start voice thread, already started!");
            false
        }
    }
}

pub(crate) fn request_end(output_buffer: ManuallyDrop<Vec<u8>>) -> bool {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(job), Some(sender)) => {
            sender.send(Message::Stop(output_buffer)).unwrap();
            match job.join() {
                Ok(job_success) => job_success,
                Err(_) => {
                    log::error!("Cannot stop voice thread, Failure Joining Thread!");
                    false
                }
            }
        }
        (_, _) => {
            log::error!("Cannot stop voice thread, not started!");
            false
        }
    }
}

pub(crate) fn request_abort() -> bool {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(job), Some(sender)) => {
            sender.send(Message::Abort).unwrap();
            match job.join() {
                Ok(_) => true,
                Err(_) => {
                    log::error!("Cannot stop voice thread, Failure Joining Thread!");
                    false
                }
            }
        }
        (_, _) => {
            log::error!("Cannot stop voice thread, not started!");
            false
        }
    }
}

fn audio_job(device_id: i32, sample_rate: i32, channels: i32, recv: Receiver<Message>) -> bool {
    let thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>> =
        Arc::new(ArrayQueue::new((INTERVALS_PER_SECOND * 30) as usize));
    let samples_per_interval = channels * sample_rate / INTERVALS_PER_SECOND;
    let input_stream = AudioStreamBuilder::new()
        .expect("Could not get Audio Stream Builder")
        .device_id(device_id)
        .direction(AudioDirection::Input)
        .sharing_mode(AudioSharingMode::Shared)
        .performance_mode(AudioPerformanceMode::LowLatency)
        .frames_per_data_callback(samples_per_interval)
        .sample_rate(sample_rate)
        .channel_count(channels)
        .format(ndk::audio::AudioFormat::PCM_I16)
        .allowed_capture_policy(AudioAllowedCapturePolicy::AllowCaptureByNone)
        .data_callback(Box::new(
            move |_audio_stream, frame_buffer, count| -> AudioCallbackResult {
                let i16_array = frame_buffer as *mut i16; //TODO try i32 or c_int if this doesn't work!
                let vec_audio_frames =
                    unsafe { Vec::from_raw_parts(i16_array, count as usize, count as usize) };
                log::info!("p");
                match thirty_second_audio_buffer.push(vec_audio_frames) {
                    Ok(()) => AudioCallbackResult::Continue,
                    Err(_) => AudioCallbackResult::Stop,
                }
            },
        ))
        .open_stream()
        .expect("Could not get AudioStream");
    log_trace_audio_stream_info(&input_stream);
    input_stream.request_start().unwrap();
    input_stream.request_stop().unwrap();
    input_stream
        .wait_for_state_change(
            AudioStreamState::Stopping,
            Duration::from_secs(1).as_nanos() as i64,
        )
        .unwrap();
    match recv.recv() {
        Ok(Message::Abort) => true,
        Ok(Message::Stop(buff)) => true,
        Err(_) => false,
    }
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
