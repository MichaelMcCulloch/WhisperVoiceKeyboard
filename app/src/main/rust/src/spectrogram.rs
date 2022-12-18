use std::sync::Arc;

use crate::{lina::dot_product, statics::WHISPER_FILTERS};
use nalgebra::Complex;
use ndk_sys::exit;
use rayon::prelude::*;

use rustfft::{num_complex::Complex32, num_traits::Zero, Fft};

const SAMPLE_RATE: usize = 16000;
const RECORDING_LEN: usize = 30;
const N_FFT: usize = 201;
const FFT_LEN: usize = 400;

const N_MEL_BINS: usize = 80;
const MEL_LEN: usize = 3000;

const HOP_LENGTH: usize = 160;

/// This method is used to generate a log mel spectrogram from a given `f32le_audio` vector. It does this by applying a window function and using an FFT process to compute the power spectrum, before computing the logmel spectrogram.
///
/// The spectrogram is also normalized to have a maximum value of `1.0`.
///
/// ## Arguments
/// - `f32le_audio`: A vector of floats of audio data in little endian format.
///
/// ## Return Value
/// A vector of floats representing the log mel spectrogram.
pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Vec<f32> {
    // Take the whisper filters lock, if it exists.
    match unsafe { WHISPER_FILTERS.take() } {
        Some(filters) => {
            let thread_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(8)
                .build()
                .unwrap();
            let fft_process = get_fft_plan(FFT_LEN);
            let mut working_buffer: Vec<f32> = vec![0.0; FFT_LEN];
            let mut mel_spectrogram_columns = Vec::with_capacity(MEL_LEN * N_MEL_BINS);
            let hann = hann_window(FFT_LEN);
            for i in 0..3000usize {
                let offset = i * HOP_LENGTH;
                for j in 0..FFT_LEN {
                    if offset + j < SAMPLE_RATE * RECORDING_LEN {
                        working_buffer[j] = hann[j] * f32le_audio[offset + j]
                    } else {
                        working_buffer[j] = 0.0;
                    }
                }
                let mut log_mel_spectrogram = thread_pool.install(|| {
                    let fft_complex_output = compute_fft(&working_buffer, &fft_process);
                    let power_spectrum = compute_power(&fft_complex_output);

                    compute_mel(&power_spectrum, &filters, N_MEL_BINS, N_FFT)
                });
                append(&mut mel_spectrogram_columns, &mut log_mel_spectrogram);
                //Reset the working buffer to all zeros.
                working_buffer.copy_from_slice(&[0.0; FFT_LEN]);
            }

            // Replace the whisper filters lock.
            unsafe { WHISPER_FILTERS.replace(filters) };

            // Normalize the mel spectrogram columns buffer.
            normalize(&mut mel_spectrogram_columns);

            // Return the mel spectrogram columns buffer.
            mel_spectrogram_columns
        }

        // If the whisper filter lock does not exist, throw a todo! error.
        None => todo!(),
    }
}

fn hann_window(fft_len: usize) -> Vec<f32> {
    (0..fft_len)
        .into_iter()
        .map(|i| 0.5 * (1.0 - (-2.0 * std::f32::consts::PI * i as f32 / fft_len as f32).cos()))
        .collect()
}

fn get_fft_plan(fft_length: usize) -> Arc<dyn Fft<f32>> {
    // Create the FFT process.
    #[cfg(target_arch = "x86_64")]
    let mut fft_planner = {
        use rustfft::FftPlanner;
        FftPlanner::new()
    };
    #[cfg(target_arch = "aarch64")]
    let mut fft_planner = {
        use rustfft::FftPlannerNeon;
        FftPlannerNeon::new().unwrap()
    };
    fft_planner.plan_fft_forward(fft_length)
}

/// Compute an FFT from a working buffer and an FFT process object.
fn compute_fft(working_buffer: &[f32], fft_process: &Arc<dyn Fft<f32>>) -> Vec<Complex32> {
    let mut fft_work_buffer: Vec<Complex32> = working_buffer
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();

    fft_process.process(&mut fft_work_buffer[..]);
    fft_work_buffer
    // fft(&working_buffer.to_vec())
}
/// Compute the power spectrum from an FFT result buffer.
fn compute_power(fft_work_buffer: &[Complex32]) -> Vec<f32> {
    let mut power_spectrum = vec![0.0; fft_work_buffer.len()];

    for i in 0..fft_work_buffer.len() {
        power_spectrum[i] = fft_work_buffer[i].norm_sqr()
    }

    // Perform doubling of the power spectrum
    for j in 1..(fft_work_buffer.len() / 2) {
        power_spectrum[j] += power_spectrum[fft_work_buffer.len() - j]
    }

    power_spectrum[0..(fft_work_buffer.len() / 2) + 1].to_vec()
}

/// Compute the log mel spectrogram from a power spectrum buffer and filters.
fn compute_mel(
    power_spectrum: &[f32],
    filters: &[Vec<f32>],
    mel_bins: usize,
    n_fft: usize,
) -> Vec<f32> {
    let mut log_mel_spectrogram = vec![0.0f32; mel_bins as usize];
    for i in 0..mel_bins {
        log_mel_spectrogram[i] = dot_product(&power_spectrum, &filters[i]);
    }

    log_mel_spectrogram
}

/// Append the log mel spectrogram to the mel spectrogram columns buffer.
fn append(mel_spectrogram_columns: &mut Vec<f32>, log_mel_spectrogram: &mut Vec<f32>) {
    mel_spectrogram_columns.extend(log_mel_spectrogram.iter());
}

/// Normalize the mel spectrogram columns buffer.
fn normalize(mel_spectrogram_columns: &mut [f32]) {
    // Compute the maximum value of the mel spectrogram columns buffer.
    let maximum_value = mel_spectrogram_columns
        .iter()
        .fold(-1e20f32, |acc, f| f.max(acc));
    mel_spectrogram_columns
        .iter_mut()
        .for_each(|x| *x = ((*x).max(1e-10).log10().max(maximum_value - 8.0) + 4.0) / 4.0);
}
