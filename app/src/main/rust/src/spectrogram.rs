use std::sync::Arc;

use crate::{statics::WHISPER_FILTERS, whisper::filters::Filters};
use nalgebra::Complex;
use ndk_sys::exit;
use rayon::prelude::*;

use rustfft::{num_complex::Complex32, Fft};
const HANN_ALPHA: f32 = 0.5;
const HANN_BETA: f32 = -2.0 * std::f32::consts::PI;
const _SAMPLE_RATE: i32 = 16000;
const N_FFT: usize = 201;
const FFT_LEN: usize = 400;

const N_MEL: usize = 80;
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
            // Set up the FFT planner.
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

            // Create the FFT process.
            let fft_process = fft_planner.plan_fft_forward(FFT_LEN);
            // Create the hann window function.
            let window_function: Vec<_> = (0..FFT_LEN)
                .into_iter()
                .map(|i| HANN_ALPHA * (1.0 - (HANN_BETA * i as f32 / FFT_LEN as f32).cos()))
                .collect();

            let npar_chunks = 4;
            let par_chunk_size = f32le_audio.len() / npar_chunks;

            let mut result = f32le_audio
                .par_chunks_exact(par_chunk_size)
                .flat_map(|chunk_of_audio| {
                    // Create a working buffer and a mel spectrogram columns buffer.
                    let mut working_buffer: Vec<f32> = vec![0.0; FFT_LEN];
                    let mut mel_spectrogram_columns =
                        Vec::with_capacity(MEL_LEN / npar_chunks * N_MEL);
                    // Process each frame of audio.
                    chunk_of_audio
                        .chunks(HOP_LENGTH)
                        .into_iter()
                        .for_each(|frame| {
                            // Apply the frame to the working buffer.
                            apply_frame(&mut working_buffer, frame);

                            // Apply the window function to the working buffer.
                            apply_window(&mut working_buffer, &window_function);

                            // Compute the FFT of the working buffer.
                            let fft_work_buffer: Vec<Complex32> =
                                compute_fft(&working_buffer, &fft_process);

                            // Compute the power spectrum of the FFT result.
                            let power_spectrum = compute_power(&fft_work_buffer);

                            // Compute the log mel spectrogram of the power spectrum result.
                            let mut log_mel_spectrogram = compute_logmel(&power_spectrum, &filters);

                            // Append to mel spectrogram columns buffer and reset the working buffer.
                            append(&mut mel_spectrogram_columns, &mut log_mel_spectrogram);
                            reset_working_buffer(&mut working_buffer);
                        });

                    // Replace the whisper filters lock.
                    mel_spectrogram_columns
                })
                .collect::<Vec<_>>();
            unsafe { WHISPER_FILTERS.replace(filters) };
            // Return the mel spectrogram columns buffer.
            // Compute the maximum value of the mel spectrogram columns buffer.
            let maximum_value = result.iter().fold(f32::MIN, |acc, f| f.max(acc));

            // Normalize the mel spectrogram columns buffer.
            normalize(&mut result, maximum_value);
            result
        }

        // If the whisper filter lock does not exist, throw a todo! error.
        None => todo!(),
    }
}

/// Apply a frame of audio to a working buffer.
fn apply_frame(working_buffer: &mut [f32], frame: &[f32]) {
    frame
        .iter()
        .enumerate()
        .for_each(|(i, &sample)| working_buffer[i] += sample);
}

/// Apply a window function to a working buffer.
fn apply_window(working_buffer: &mut [f32], window_function: &[f32]) {
    working_buffer
        .iter_mut()
        .enumerate()
        .for_each(|(i, sample)| *sample *= window_function[i]);
}

/// Compute an FFT from a working buffer and an FFT process object.
fn compute_fft(working_buffer: &[f32], fft_process: &Arc<dyn Fft<f32>>) -> Vec<Complex32> {
    let mut fft_work_buffer: Vec<Complex32> = working_buffer
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();

    fft_process.process(&mut fft_work_buffer[..]);

    fft_work_buffer
}

/// Compute the power spectrum from an FFT result buffer.
fn compute_power(fft_work_buffer: &[Complex32]) -> Vec<f32> {
    let mut power_spectrum = vec![0.0; FFT_LEN as usize];

    (0..FFT_LEN as usize)
        .into_iter()
        .for_each(|i| power_spectrum[i] = fft_work_buffer[i].norm_sqr());

    // Perform doubling of the power spectrum
    (1..FFT_LEN as usize / 2)
        .into_iter()
        .for_each(|j| power_spectrum[j] += power_spectrum[FFT_LEN as usize - j]);

    power_spectrum
}

/// Compute the log mel spectrogram from a power spectrum buffer and filters.
fn compute_logmel(power_spectrum: &[f32], filters: &Filters) -> Vec<f32> {
    let mut log_mel_spectrogram = vec![0.0f32; N_MEL as usize];
    (0..N_MEL as usize).into_iter().for_each(|i| {
        let left = &power_spectrum[..];
        let right = &filters.data[i * N_FFT..(i + 1) * N_FFT];

        log_mel_spectrogram[i] = dot_product(&left[0..N_FFT], right);
    });

    log_mel_spectrogram
}

// #[cfg(target_arch = "x86_64")]
/// Calculates the dot product of two slices of `f32` values.
///
/// # Parameters
///
/// * `left` - The left slice of `f32` values.
/// * `right` - The right slice of `f32` values.
///
/// # Returns
///
/// The dot product of the two slices.
///
/// # Examples
///
/// ```
/// let left = [1.0f32, 2.0f32, 3.0f32];
/// let right = [4.0f32, 5.0f32, 6.0f32];
///
/// let result = dot_product(&left, &right);
/// assert_eq!(result, 32.0f32);
/// ```
fn dot_product(left: &[f32], right: &[f32]) -> f32 {
    let mut sum = 0.0;

    for (l, r) in left.iter().zip(right.iter()) {
        sum += l * r;
    }

    sum
}
#[cfg(target_arch = "aarch64")]
// #[cfg(target_arch = "aarch64")]
/// I tried to beat the compiler. I lost.
/// Calculates the dot product of two slices of `f32` values.
///
/// # Parameters
///
/// * `left` - The left slice of `f32` values.
/// * `right` - The right slice of `f32` values.
///
/// # Returns
///
/// The dot product of the two slices.
///
/// # Examples
///
/// ```
/// let left = [1.0f32, 2.0f32, 3.0f32];
/// let right = [4.0f32, 5.0f32, 6.0f32];
///
/// let result = dot_product(&left, &right);
/// assert_eq!(result, 32.0f32);
/// ```
fn dot_productSlow(left: &[f32], right: &[f32]) -> f32 {
    // log::info!("{}x{}", left.len(), right.len());
    use std::arch::aarch64::{vdupq_n_f32, vfmaq_f32, vgetq_lane_f32, vld1q_f32};
    // We'll pad the arrays so that their lengths are divisible by 4
    let pad_length_l = 4 - (left.len() % 4);
    let pad_length_r = 4 - (right.len() % 4);
    // Create the padded arrays
    let mut pad_left = vec![0.0f32; left.len() + pad_length_l];
    let mut pad_right = vec![0.0f32; right.len() + pad_length_r];
    // Copy the contents of the original arrays into the padded arrays.
    pad_left[0..left.len()].copy_from_slice(left);
    pad_right[0..right.len()].copy_from_slice(right);
    // Initialize accumulator to 0.0
    let zero = unsafe { vdupq_n_f32(0.0f32) };
    // Perform the dot product using 4-element SIMD instructions
    let result = pad_left
        .chunks_exact(4)
        .zip(pad_right.chunks_exact(4))
        .into_iter()
        .fold(zero, |acc, (left, right)| unsafe {
            // Load the 4 elements for each array into a single SIMD vector
            let l = vld1q_f32(left.as_ptr());
            let r = vld1q_f32(right.as_ptr());
            // Perform 4-element multiply-accumulate
            vfmaq_f32(acc, l, r)
        });

    // Sum the 4 elements in the result vector
    let result = unsafe {
        vgetq_lane_f32(result, 0)
            + vgetq_lane_f32(result, 1)
            + vgetq_lane_f32(result, 2)
            + vgetq_lane_f32(result, 3)
    };
    result
}

/// Append the log mel spectrogram to the mel spectrogram columns buffer.
fn append(mel_spectrogram_columns: &mut Vec<f32>, log_mel_spectrogram: &mut Vec<f32>) {
    mel_spectrogram_columns.extend(log_mel_spectrogram.iter());
}

/// Reset the working buffer to all zeros.
fn reset_working_buffer(working_buffer: &mut [f32]) {
    working_buffer.copy_from_slice(&[0.0; FFT_LEN]);
}

/// Normalize the mel spectrogram columns buffer.
fn normalize(mel_spectrogram_columns: &mut [f32], maximum_value: f32) {
    mel_spectrogram_columns
        .iter_mut()
        .for_each(|x| *x = ((*x).max(1e-10).log10().max(maximum_value - 8.0) + 4.0) / 4.0);
}
