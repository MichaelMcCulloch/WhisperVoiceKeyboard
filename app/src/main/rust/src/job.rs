use std::{
    fs::File,
    path::PathBuf,
    str::FromStr,
    sync::{mpsc::Receiver, Arc},
};

use ac_ffmpeg::{
    codec::{
        audio::{
            AudioEncoder, AudioFrame, AudioFrameMut, AudioResampler, ChannelLayout, SampleFormat,
        },
        AudioCodecParameters, AudioCodecParametersBuilder, CodecParameters, Encoder,
    },
    format::{
        io::IO,
        muxer::{Muxer, MuxerBuilder, OutputFormat},
    },
    packet::Packet,
    Error,
};
use crossbeam_queue::ArrayQueue;
use ndk::audio::{
    AudioAllowedCapturePolicy, AudioCallbackResult, AudioDirection, AudioFormat,
    AudioPerformanceMode, AudioSharingMode, AudioStream, AudioStreamBuilder,
};

use crate::{Message, CACHE_DIR};
const INTERVALS_PER_SECOND: usize = 10;
const RECORDING_IN_SECONDS: usize = 30;
const RECORDING_FORMAT_S16_NDK: AudioFormat = AudioFormat::PCM_I16;
const RECORDING_FORMAT_S16_FFMPEG: &str = "s16";
const RESAMPLE_TARGET_HZ: u32 = 16000;
const RESAMPLE_TARGET_CHANNELS: u32 = 1;

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

                // write_before(
                //     &ffmpeg_audio_frame.clone(),
                //     channels as u32,
                //     sample_rate as u32,
                // );

                let mut resampler = get_resampler(channels, sample_rate);
                resampler.push(ffmpeg_audio_frame).unwrap();
                let frame = resampler.take().unwrap().unwrap();

                // write_after(&frame.clone());

                break Some(Vec::from(&frame.planes()[0].data()[0..960000]));
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

fn write_after(frame: &AudioFrame) {
    let output_format = OutputFormat::guess_from_file_name("audio.wav")
        .ok_or_else(|| Error::new(format!("unable to guess output format for file: audio.wav",)))
        .unwrap();
    let mut encoder = AudioEncoder::builder("pcm_s16le")
        .unwrap()
        .bit_rate(256)
        .sample_format(SampleFormat::from_str(RECORDING_FORMAT_S16_FFMPEG).unwrap())
        .channel_layout(ChannelLayout::from_channels(RESAMPLE_TARGET_CHANNELS).unwrap())
        .sample_rate(RESAMPLE_TARGET_HZ)
        .build()
        .unwrap();
    let codec = encoder.codec_parameters();
    let cache_dir = unsafe { CACHE_DIR.clone().unwrap() };
    let mut pb = PathBuf::from(cache_dir);
    pb.push("After.wav");
    let file = File::create(pb.clone()).unwrap();
    let io = IO::from_seekable_write_stream(file);
    let mut muxer_builder = Muxer::builder();
    muxer_builder
        .add_stream(&CodecParameters::from(codec.clone()))
        .unwrap();
    let mut muxer = muxer_builder.build(io, output_format).unwrap();
    encoder.push(frame.clone()).unwrap();
    muxer
        .push(encoder.take().unwrap().unwrap().with_stream_index(0))
        .unwrap();
    log::info!("I think I wrote a 16Khz wave file to {}", pb.display());
}

fn write_before(frame: &AudioFrame, channels: u32, sample_rate: u32) {
    let output_format = OutputFormat::guess_from_file_name("audio.wav")
        .ok_or_else(|| Error::new(format!("unable to guess output format for file: audio.wav",)))
        .unwrap();
    let mut encoder = AudioEncoder::builder("pcm_s16le")
        .unwrap()
        .bit_rate(256)
        .sample_format(SampleFormat::from_str(RECORDING_FORMAT_S16_FFMPEG).unwrap())
        .channel_layout(ChannelLayout::from_channels(channels).unwrap())
        .sample_rate(sample_rate)
        .build()
        .unwrap();
    let codec = encoder.codec_parameters();
    let cache_dir = unsafe { CACHE_DIR.clone().unwrap() };
    let mut pb = PathBuf::from(cache_dir);
    pb.push("Before.wav");
    let file = File::create(pb.clone()).unwrap();
    let io = IO::from_seekable_write_stream(file);
    let mut muxer_builder = Muxer::builder();
    muxer_builder
        .add_stream(&CodecParameters::from(codec.clone()))
        .unwrap();
    let mut muxer = muxer_builder.build(io, output_format).unwrap();
    encoder.push(frame.clone()).unwrap();
    muxer
        .push(encoder.take().unwrap().unwrap().with_stream_index(0))
        .unwrap();
    log::info!("I think I wrote a 48Khz wave file to {}", pb.display());
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
        .format(RECORDING_FORMAT_S16_NDK)
        .allowed_capture_policy(AudioAllowedCapturePolicy::AllowCaptureByNone)
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

fn get_resampler(channels: i32, sample_rate: i32) -> AudioResampler {
    AudioResampler::builder()
        .source_channel_layout(ChannelLayout::from_channels(channels as u32).unwrap())
        .source_sample_format(SampleFormat::from_str(RECORDING_FORMAT_S16_FFMPEG).unwrap())
        .source_sample_rate(sample_rate as u32)
        .target_channel_layout(ChannelLayout::from_channels(RESAMPLE_TARGET_CHANNELS).unwrap())
        .target_sample_format(SampleFormat::from_str(RECORDING_FORMAT_S16_FFMPEG).unwrap())
        .target_sample_rate(RESAMPLE_TARGET_HZ)
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
