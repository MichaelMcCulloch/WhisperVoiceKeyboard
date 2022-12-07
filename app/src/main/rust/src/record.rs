use std::{
    mem::ManuallyDrop,
    slice,
    str::FromStr,
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
    time::Duration,
};

use ac_ffmpeg::codec::audio::{AudioFrameMut, AudioResampler, ChannelLayout, SampleFormat};
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

const INTERVALS_PER_SECOND: usize = 10;
const RECORDING_IN_SECONDS: usize = 30;

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
            log::info!("Starting Voice Thread");
            true
        }
        (_, _) => {
            log::error!("Cannot start voice thread, already started!");
            false
        }
    }
}

pub(crate) fn request_end() -> Option<Vec<u8>> {
    match unsafe {
        (
            AUDIO_PROCESSING_THREAD.take(),
            AUDIO_PROCESSING_THREAD_MESSENGER.take(),
        )
    } {
        (Some(job), Some(sender)) => {
            sender.send(Message::Stop).unwrap();
            let ret = match job.join() {
                Ok(job_success) => job_success,
                Err(_) => {
                    log::error!("Cannot stop voice thread, Failure Joining Thread!");
                    None
                }
            };
            log::info!("Stopping Voice Thread");

            ret
        }
        (_, _) => {
            log::error!("Cannot stop voice thread, not started!");
            None
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

fn audio_job(
    device_id: i32,
    sample_rate: i32,
    channels: i32,
    recv: Receiver<Message>,
) -> Option<Vec<u8>> {
    let thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>> = Arc::new(ArrayQueue::new(
        (INTERVALS_PER_SECOND * RECORDING_IN_SECONDS) as usize,
    ));
    let tsab = thirty_second_audio_buffer.clone();
    let samples_per_interval = channels * sample_rate / INTERVALS_PER_SECOND as i32;
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
                let vec_audio_frames = unsafe { slice::from_raw_parts(i16_array, count as usize) };
                match tsab.push(vec_audio_frames.to_vec()) {
                    Ok(()) => AudioCallbackResult::Continue,
                    Err(_) => AudioCallbackResult::Stop,
                }
            },
        ))
        .open_stream()
        .expect("Could not get AudioStream");

    log_trace_audio_stream_info(&input_stream);
    start_recording(&input_stream);

    let ret = loop {
        match recv.recv() {
            Ok(Message::Stop) => {
                stop_recording(&input_stream);
                log::info!("a");
                // convert the input signal to 16khz mono

                //Produce some silence within FFMPEG
                let mut silence = AudioFrameMut::silence(
                    &ChannelLayout::from_channels(channels as u32).unwrap(),
                    SampleFormat::from_str("s16").unwrap(),
                    sample_rate as u32,
                    sample_rate as usize * 30,
                );
                log::info!("b");
                let mut planes = silence.planes_mut();
                let plane_data = planes[0].data_mut();
                let (__pre, plane_data_i16, _post) = unsafe { plane_data.align_to_mut::<i16>() };
                let mut start = 0;
                log::info!("c");
                while let Some(vec_frames) = thirty_second_audio_buffer.pop() {
                    // audio_buffer.append(&mut vec_frames)
                    plane_data_i16[start..vec_frames.len() - 1].copy_from_slice(&vec_frames);
                    start = vec_frames.len();
                }
                log::info!("d");
                let resampler = AudioResampler::builder()
                    .source_channel_layout(ChannelLayout::from_channels(channels as u32).unwrap())
                    .source_sample_format(SampleFormat::from_str("s16").unwrap())
                    .source_sample_rate(sample_rate as u32)
                    .target_channel_layout(ChannelLayout::from_channels(1).unwrap())
                    .target_sample_format(SampleFormat::from_str("s16").unwrap())
                    .target_sample_rate(16000)
                    .build()
                    .unwrap();
                log::info!("e");
                let (_, buff, _) = unsafe { plane_data_i16.align_to::<u8>() };
                break Some(Vec::from(buff));
            }
            Ok(Message::Abort) => {
                stop_recording(&input_stream);
                break None;
            }
            Ok(Message::Resume) => {
                input_stream.request_start().unwrap();
                continue;
            }
            Ok(Message::Pause) => {
                input_stream.request_pause().unwrap();
                continue;
            }
            Err(_) => {
                stop_recording(&input_stream);
                break None;
            }
        }
    };
    log::info!("I see no issue here");
    ret
}

fn get_input_stream(
    channels: i32,
    sample_rate: i32,
    device_id: i32,
    thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>>,
) -> AudioStream {
    let samples_per_interval = channels * sample_rate / INTERVALS_PER_SECOND as i32;
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
                log::info!("Ok");
                match thirty_second_audio_buffer.clone().push(vec_audio_frames) {
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

fn start_recording(input_stream: &AudioStream) {
    input_stream.request_start().unwrap();
}
fn stop_recording(input_stream: &AudioStream) {
    input_stream.request_stop().unwrap();
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
