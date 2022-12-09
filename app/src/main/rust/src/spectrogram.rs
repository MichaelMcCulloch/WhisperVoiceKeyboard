#[cfg(target_arch = "x86_64")]
use rustfft::FftPlanner;
#[cfg(target_arch = "aarch64")]
use rustfft::FftPlannerNeon;

use rustfft::{num_complex::Complex32, num_traits::Zero};

// Create a new FFT planner
use crate::{
    consts::{WHISPER_HOP_LENGTH, WHISPER_N_FFT},
    whisper::mel::Mel,
};

/// A fucking machien wrote this;
pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Mel {
    let window_size = WHISPER_N_FFT; // FFT window size in samples
    let hop_size = WHISPER_HOP_LENGTH; // Hop size in samples

    // Create a new FFT planner
    #[cfg(target_arch = "x86_64")]
    let mut planner = FftPlanner::new();
    #[cfg(target_arch = "aarch64")]
    let mut planner = FftPlannerNeon::new().unwrap();

    // Create an FFT object with the specified window size
    let fft = planner.plan_fft_forward(window_size);

    // Set up the Hann window
    let window: Vec<f32> = (0..window_size)
        .map(|i| 0.5 * (1.0 - (-2.0 * std::f32::consts::PI * i as f32 / window_size as f32).cos()))
        .collect();

    // Create a buffer to hold the overlapping audio frames
    let mut buffer: Vec<f32> = vec![0.0; window_size];

    let mut columns = Vec::with_capacity(3000);

    let num_threads = 8;
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
    // Iterate through the audio stream, computing the STFT for each frame of audio
    for frame in f32le_audio.chunks(hop_size) {
        // Overlap-add the current audio frame onto the buffer
        for (i, sample) in frame.iter().enumerate() {
            buffer[i] += sample;
        }

        // Apply the Hann window to the buffer
        for (i, sample) in buffer.iter_mut().enumerate() {
            *sample *= window[i];
        }

        // Convert the samples in the buffer to complex numbers
        let mut input: Vec<Complex32> = buffer
            .iter()
            .map(|&x| Complex32 { re: x, im: 0.0 })
            .collect();

        // Create a buffer to hold the STFT output
        let mut output: Vec<Complex32> = vec![Complex32 { re: 0.0, im: 0.0 }; window_size];

        // Compute the STFT of the audio framefn process(&self, buffer: &mut [Complex<T>]) {
        let mut scratch = vec![Complex32::zero(); fft.get_outofplace_scratch_len()];
        thread_pool.install(|| {
            fft.process_outofplace_with_scratch(&mut input[..], &mut output[..], &mut scratch);
        });
        // Use the STFT output here (e.g. for spectral analysis or resynthesis)
        columns.push(output);
        // Shift the samples in the buffer to prepare for the next frame
        for i in 0..hop_size {
            buffer[i] = buffer[i + hop_size];
        }
        for i in hop_size..window_size {
            buffer[i] = 0.0;
        }
    }
    log::info!("{}", columns.len());
    Mel::new(0, 0, vec![])
}
