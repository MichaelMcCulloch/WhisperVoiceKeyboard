use std::{
    str::FromStr,
    sync::{mpsc::Receiver, Arc},
};

use ac_ffmpeg::codec::audio::{
    AudioFrame, AudioFrameMut, AudioResampler, ChannelLayout, SampleFormat,
};
use crossbeam_queue::ArrayQueue;
use ndk::audio::{
    AudioAllowedCapturePolicy, AudioCallbackResult, AudioDirection, AudioFormat, AudioSharingMode,
    AudioStream, AudioStreamBuilder,
};

use crate::{spectrogram::log_mel_spectrogram, Message};
const CALLBACK_INTERVALS_PER_SECOND: usize = 10;
const THIRTY_SECONDS: usize = 30;
const RECORDING_FORMAT_S16_NDK: AudioFormat = AudioFormat::PCM_I16;
/// As described by [https://ffmpeg.org](https://ffmpeg.org/doxygen/2.3/group__lavu__sampfmts.html#gaf9a51ca15301871723577c730b5865c5)
const RECORDING_FORMAT_S16_FFMPEG: &str = "s16";
const RECORDING_FORMAT_F32_FFMPEG: &str = "flt";

const SIXTEEN_KHZ: usize = 16000;
const MONO_CHANNEL: usize = 1;

const U8_PER_F32: usize = 4;

const U8_COUNT_FOR_30SECONDS_16KHZ_F32_AUDIO: usize =
    SIXTEEN_KHZ * THIRTY_SECONDS * MONO_CHANNEL * U8_PER_F32;

pub(crate) fn audio_job(
    device_id: i32,
    sample_rate: i32,
    channels: i32,
    recv: Receiver<Message>,
) -> Option<Vec<u8>> {
    let thirty_second_audio_buffer: Arc<ArrayQueue<Vec<i16>>> = Arc::new(ArrayQueue::new(
        (CALLBACK_INTERVALS_PER_SECOND * THIRTY_SECONDS) as usize,
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

                let planes = frame.planes();
                let audio_data = &planes[0].data()[0..U8_COUNT_FOR_30SECONDS_16KHZ_F32_AUDIO];

                let (_pre, f32le_audio, _post) = unsafe { audio_data.align_to::<f32>() };

                assert!(_pre.is_empty() && _post.is_empty());
                let mel = log_mel_spectrogram(f32le_audio);
                let (_pre, f32le_spectrogram, _post) = unsafe { mel.align_to::<u8>() };
                assert!(_pre.is_empty() && _post.is_empty());
                break Some(Vec::from(f32le_spectrogram));
            }
            Ok(Message::Abort) => {
                input_stream.request_stop().unwrap();
                break None;
            }
            Err(_) => {
                input_stream.request_stop().unwrap();
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
    let samples_per_interval = channels * sample_rate / CALLBACK_INTERVALS_PER_SECOND as i32;

    let input_stream = AudioStreamBuilder::new()
        .expect("Could not get Audio Stream Builder")
        .device_id(device_id)
        .direction(AudioDirection::Input)
        .sharing_mode(AudioSharingMode::Shared)
        .frames_per_data_callback(samples_per_interval)
        .sample_rate(sample_rate)
        .channel_count(channels)
        .format(RECORDING_FORMAT_S16_NDK)
        // .allowed_capture_policy(AudioAllowedCapturePolicy::AllowCaptureByNone)
        .data_callback(Box::new(
            move |_audio_stream, frame_buffer, count| -> AudioCallbackResult {
                // Android is LITTLE ENDIAN; ffmpeg will use native endianness -> can just realign and it will work
                let i16le_frames = frame_buffer as *mut i16;
                let vec_audio_frames =
                    unsafe { std::slice::from_raw_parts(i16le_frames, count as usize) };
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
/// This Resampler will convert s16, N channel, X  kHz format to f32, 1 channel, 16kHz
fn get_resampler(channels: i32, sample_rate: i32) -> AudioResampler {
    AudioResampler::builder()
        .source_channel_layout(ChannelLayout::from_channels(channels as u32).unwrap())
        .source_sample_format(SampleFormat::from_str(RECORDING_FORMAT_S16_FFMPEG).unwrap())
        .source_sample_rate(sample_rate as u32)
        .target_channel_layout(ChannelLayout::from_channels(MONO_CHANNEL as u32).unwrap())
        .target_sample_format(SampleFormat::from_str(RECORDING_FORMAT_F32_FFMPEG).unwrap())
        .target_sample_rate(SIXTEEN_KHZ as u32)
        .build()
        .unwrap()
}

fn wrap_audio_in_av_frame(
    channels: i32,
    sample_rate: i32,
    thirty_second_audio_buffer: &Arc<ArrayQueue<Vec<i16>>>,
) -> AudioFrame {
    let mut empty_ffmpeg_frame = AudioFrameMut::silence(
        &ChannelLayout::from_channels(channels as u32).unwrap(),
        SampleFormat::from_str(RECORDING_FORMAT_S16_FFMPEG).unwrap(),
        sample_rate as u32,
        sample_rate as usize * 30,
    );
    let mut planes = empty_ffmpeg_frame.planes_mut();
    let plane_data = planes[0].data_mut();
    let mut start = 0;
    while let Some(i16le_frames) = thirty_second_audio_buffer.pop() {
        let (_pre, bytes, _post) = unsafe { i16le_frames.align_to::<u8>() };

        let target = &mut plane_data[start..(start + bytes.len())];
        target.copy_from_slice(bytes);

        start += bytes.len();
    }
    empty_ffmpeg_frame.freeze()
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
