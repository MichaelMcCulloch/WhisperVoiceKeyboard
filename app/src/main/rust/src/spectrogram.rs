use std::process::exit;

use crate::{
    consts::{
        WHISPER_CHUNK_SIZE, WHISPER_FFT_LEN, WHISPER_HOP_LENGTH, WHISPER_MEL_LEN, WHISPER_N_FFT,
        WHISPER_N_MEL, WHISPER_SAMPLE_RATE,
    },
    statics::WHISPER_FILTERS,
    whisper::mel::Mel,
};
use nalgebra::Complex;
use ndarray::Array2;
use ndarray_ndimage::convolve;
#[cfg(target_arch = "x86_64")]
use rustfft::FftPlanner;
#[cfg(target_arch = "aarch64")]
use rustfft::FftPlannerNeon;
use rustfft::{num_complex::Complex32, num_traits::Zero};
const HANN_ALPHA: f32 = 0.5;
const HANN_BETA: f32 = -2.0 * std::f32::consts::PI;
const SAMPLE_RATE: i32 = 16000;
const N_FFT: usize = 201;

const N_MEL: usize = 80;
const MEL_LEN: usize = 3000;

const HOP_LENGTH: usize = 160;
const CHUNK_SIZE: i32 = 30;
// This function logs a Mel Spectrogram, which is a representation of audio signal power
// on a set of Mel frequency bands. It uses FFT (Fast Fourier Transform) and Hann windowing
// to calculate the power values.
pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Vec<f32> {
    // Determine whether the code is running on an x86_64 or aarch64 architecture
    // and create the appropriate FFT planner
    #[cfg(target_arch = "x86_64")]
    let mut fft_planner = FftPlanner::new();
    #[cfg(target_arch = "aarch64")]
    let mut fft_planner = FftPlannerNeon::new().unwrap();

    // Create the FFT process, which is used to calculate the FFT of the audio signal
    let fft_process = fft_planner.plan_fft_forward(N_FFT);

    // Create a Hann window function, which is used to reduce artifacts in the power spectrum calculation
    let window_function = (0..HOP_LENGTH)
        .into_iter()
        .map(|i| HANN_ALPHA * (1.0 - (HANN_BETA * i as f32 / HOP_LENGTH as f32).cos()))
        .collect::<Vec<_>>();

    // Create a working buffer, which is used to store the audio signal
    let mut working_buffer: Vec<f32> = vec![0.0; HOP_LENGTH];

    // Create a vector to store the Mel spectrogram columns
    let mut mel_spectrogram_columns = Vec::with_capacity(MEL_LEN * N_MEL);

    // Iterate over the audio signal frame by frame
    match unsafe { WHISPER_FILTERS.take() } {
        Some(filters) => {
            // Iterate through the audio stream, computing the STFT for each frame of audio
            f32le_audio
                .chunks(HOP_LENGTH)
                .into_iter()
                .for_each(|frame| {
                    // and add each sample to the working buffer
                    frame.iter().enumerate().for_each(|(i, &sample)| {
                        working_buffer[i] += sample;
                    });

                    // Multiply each sample in the working buffer with the window function
                    working_buffer
                        .iter_mut()
                        .enumerate()
                        .for_each(|(i, sample)| {
                            *sample *= window_function[i];
                        });

                    // Create an FFT work buffer and pad it with zeroes for the FFT process
                    let mut fft_work_buffer: Vec<Complex32> = working_buffer
                        .iter()
                        .map(|&x| Complex { re: x, im: 0.0 })
                        .collect();
                    let zeroes = &[Complex32::new(0.0f32, 0.0); N_FFT - HOP_LENGTH];
                    fft_work_buffer.extend(zeroes);
                    fft_process.process(&mut fft_work_buffer[..]);

                    // Create a power spectrum by calculating the norm of the FFT buffer
                    let mut power_spectrum = vec![0.0; N_FFT as usize];
                    (0..N_FFT as usize).into_iter().for_each(|i| {
                        power_spectrum[i] = fft_work_buffer[i].norm();
                    });

                    // Create a log Mel spectrogram by multiplying the power spectrum with the filter values
                    let mut log_mel_spectrogram = vec![0.0f32; N_MEL as usize];
                    (0..N_MEL as usize).into_iter().for_each(|i| {
                        (0..N_FFT).into_iter().for_each(|j| {
                            log_mel_spectrogram[i] +=
                                filters.data[i * N_FFT + j] * power_spectrum[j];
                        });
                    });

                    // Add the log Mel spectrogram column to the vector
                    mel_spectrogram_columns.extend(log_mel_spectrogram);

                    // Reset the working buffer
                    working_buffer.copy_from_slice(&[0.0; HOP_LENGTH]);
                });

            // Calculate the maximum value of the Mel spectrogram
            let maximum_value = mel_spectrogram_columns
                .iter()
                .fold(f32::MIN, |acc, f| f.max(acc));

            // Map the values of the Mel spectrogram to a range of 0-1
            mel_spectrogram_columns.iter_mut().for_each(|x| {
                *x = ((*x).max(1e-10).log10().max(maximum_value - 8.0) + 4.0) / 4.0;
            });

            // Return the Mel spectrogram columns

            unsafe { WHISPER_FILTERS.replace(filters) };
        }
        None => todo!(),
    }
    // Return the log Mel-frequency spectrogram
    mel_spectrogram_columns
}
