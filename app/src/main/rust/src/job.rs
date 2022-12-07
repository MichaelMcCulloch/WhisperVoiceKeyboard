use std::{
    str::FromStr,
    sync::{mpsc::Receiver, Arc},
};

use ac_ffmpeg::codec::audio::{
    AudioFrame, AudioFrameMut, AudioResampler, ChannelLayout, SampleFormat,
};
use crossbeam_queue::ArrayQueue;
use ndk::audio::{
    AudioAllowedCapturePolicy, AudioCallbackResult, AudioDirection, AudioPerformanceMode,
    AudioSharingMode, AudioStream, AudioStreamBuilder,
};

use crate::Message;
const INTERVALS_PER_SECOND: usize = 10;
const RECORDING_IN_SECONDS: usize = 30;

pub(crate) fn audio_job(
    device_id: i32,
    sample_rate: i32,
    channels: i32,
    recv: Receiver<Message>,
) -> Option<Vec<u8>> {
    let thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>> = Arc::new(ArrayQueue::new(
        (INTERVALS_PER_SECOND * RECORDING_IN_SECONDS) as usize,
    ));
    let input_stream = get_audio_stream(
        device_id,
        sample_rate,
        channels,
        thirty_second_audio_buffer.clone(),
    );

    start_recording(&input_stream);

    loop {
        match recv.recv() {
            Ok(Message::Stop) => {
                stop_recording(&input_stream);

                let ffmpeg_audio_frame =
                    wrap_audio_in_av_frame(channels, sample_rate, &thirty_second_audio_buffer);

                let mut resampler = get_resampler(channels, sample_rate);
                resampler.push(ffmpeg_audio_frame).unwrap();
                let frame = resampler.take().unwrap().unwrap();

                break Some(Vec::from(frame.planes()[0].data()));
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
    }
}

fn get_audio_stream(
    device_id: i32,
    sample_rate: i32,
    channels: i32,
    tsab: Arc<ArrayQueue<Vec<i16>>>,
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
                    unsafe { std::slice::from_raw_parts(i16_array, count as usize) };
                match tsab.push(vec_audio_frames.to_vec()) {
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

fn get_resampler(channels: i32, sample_rate: i32) -> AudioResampler {
    AudioResampler::builder()
        .source_channel_layout(ChannelLayout::from_channels(channels as u32).unwrap())
        .source_sample_format(SampleFormat::from_str("s16").unwrap())
        .source_sample_rate(sample_rate as u32)
        .target_channel_layout(ChannelLayout::from_channels(1).unwrap())
        .target_sample_format(SampleFormat::from_str("s16").unwrap())
        .target_sample_rate(16000)
        .build()
        .unwrap()
}

fn wrap_audio_in_av_frame(
    channels: i32,
    sample_rate: i32,
    thirty_second_audio_buffer: &Arc<ArrayQueue<Vec<i16>>>,
) -> AudioFrame {
    let mut silence = AudioFrameMut::silence(
        &ChannelLayout::from_channels(channels as u32).unwrap(),
        SampleFormat::from_str("s16").unwrap(),
        sample_rate as u32,
        sample_rate as usize * 30,
    );
    let mut planes = silence.planes_mut();
    let plane_data = planes[0].data_mut();
    let mut start = 0;
    while let Some(vec_frames) = thirty_second_audio_buffer.pop() {
        let (_pre, bytes, _post) = unsafe { vec_frames.align_to::<u8>() };

        let target = &mut plane_data[start..(start + bytes.len())];
        target.copy_from_slice(bytes);

        start = bytes.len();
    }
    silence.freeze()
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
