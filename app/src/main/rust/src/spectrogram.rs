use std::sync::Arc;

use crate::{statics::WHISPER_FILTERS, whisper::filters::Filters};
use nalgebra::Complex;
use rustfft::{num_complex::Complex32, Fft};
const HANN_ALPHA: f32 = 0.5;
const HANN_BETA: f32 = -2.0 * std::f32::consts::PI;
const SAMPLE_RATE: i32 = 16000;
const N_FFT: usize = 201;

const N_MEL: usize = 80;
const MEL_LEN: usize = 3000;

const HOP_LENGTH: usize = 160;
// This function logs a Mel Spectrogram, which is a representation of audio signal power
// on a set of Mel frequency bands. It uses FFT (Fast Fourier Transform) and Hann windowing
// to calculate the power values.
/* pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Vec<f32> {
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
} */

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
    let fft_process = fft_planner.plan_fft_forward(N_FFT);

    // Create the hann window function.
    let window_function: Vec<_> = (0..HOP_LENGTH)
        .into_iter()
        .map(|i| HANN_ALPHA * (1.0 - (HANN_BETA * i as f32 / HOP_LENGTH as f32).cos()))
        .collect();

    // Create a working buffer and a mel spectrogram columns buffer.
    let mut working_buffer: Vec<f32> = vec![0.0; HOP_LENGTH];
    let mut mel_spectrogram_columns = Vec::with_capacity(MEL_LEN * N_MEL);

    // Take the whisper filters lock, if it exists.
    match unsafe { WHISPER_FILTERS.take() } {
        Some(filters) => {
            // Process each frame of audio.
            f32le_audio
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

            // Compute the maximum value of the mel spectrogram columns buffer.
            let maximum_value = mel_spectrogram_columns
                .iter()
                .fold(f32::MIN, |acc, f| f.max(acc));

            // Normalize the mel spectrogram columns buffer.
            normalize(&mut mel_spectrogram_columns, maximum_value);

            // Replace the whisper filters lock.
            unsafe { WHISPER_FILTERS.replace(filters) };
        }

        // If the whisper filter lock does not exist, throw a todo! error.
        None => todo!(),
    }

    // Return the mel spectrogram columns buffer.
    mel_spectrogram_columns
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

    let zeroes = &[Complex32::new(0.0f32, 0.0); N_FFT - HOP_LENGTH];

    fft_work_buffer.extend(zeroes);

    fft_process.process(&mut fft_work_buffer[..]);

    fft_work_buffer
}

/// Compute the power spectrum from an FFT result buffer.
fn compute_power(fft_work_buffer: &[Complex32]) -> Vec<f32> {
    let mut power_spectrum = vec![0.0; N_FFT as usize];

    (0..N_FFT as usize)
        .into_iter()
        .for_each(|i| power_spectrum[i] = fft_work_buffer[i].norm());

    power_spectrum
}

/// Compute the log mel spectrogram from a power spectrum buffer and filters.
fn compute_logmel(power_spectrum: &[f32], filters: &Filters) -> Vec<f32> {
    let mut log_mel_spectrogram = vec![0.0f32; N_MEL as usize];

    // Use dot product for efficient computation
    (0..N_MEL as usize).into_iter().for_each(|i| {
        let filter = &filters.data[i * N_FFT..(i + 1) * N_FFT];

        let mut j = 0;

        #[cfg(target_arch = "x86_64")]
        {
            use std::arch::x86_64::*;

            // Process 8 floats at a time, looping until the end of the filter is reached
            while j + 8 < N_FFT {
                unsafe {
                    // Loads the power_spectrum into a vector of 8 floats
                    let power = _mm256_loadu_ps(power_spectrum.as_ptr());
                    // Loads the filter into a vector of 8 floats
                    let filter = _mm256_loadu_ps(filter.as_ptr());
                    // Calculates the dot product of the two vectors, with 8 bits of precision
                    let result = _mm256_dp_ps(power, filter, 0xFF);
                    // Converts the result to a single float value and stores it in log_mel_spectrogram
                    log_mel_spectrogram[i] += _mm256_cvtss_f32(result);
                }
                j += 8;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            use std::arch::aarch64::*;

            // Process 4 floats at a time, looping until the end of the filter is reached
            while j + 4 < N_FFT {
                unsafe {
                    // Loads the power_spectrum into a vector of 4 floats
                    let power = vld1q_f32(power_spectrum.as_ptr());
                    // Loads the filter into a vector of 4 floats
                    let filter = vld1q_f32(filter.as_ptr());
                    // Calculates the fused multiply-add (FMA) of the two vectors, with 4 bits of precision
                    let result = vfmaq_f32(vdupq_n_f32(0.0), power, filter);
                    // Gets the first lane from the result vector and adds it to log_mel_spectrogram
                    log_mel_spectrogram[i] += vgetq_lane_f32(result, 0);
                }
                j += 4;
            }
        }

        // Process remaining elements one by one
        while j < N_FFT {
            log_mel_spectrogram[i] += power_spectrum[j] * filter[j];

            j += 1;
        }
    });

    log_mel_spectrogram
}

/// Append the log mel spectrogram to the mel spectrogram columns buffer.
fn append(mel_spectrogram_columns: &mut Vec<f32>, log_mel_spectrogram: &mut Vec<f32>) {
    mel_spectrogram_columns.extend(log_mel_spectrogram.iter());
}

/// Reset the working buffer to all zeros.
fn reset_working_buffer(working_buffer: &mut [f32]) {
    working_buffer.copy_from_slice(&[0.0; HOP_LENGTH]);
}

/// Normalize the mel spectrogram columns buffer.
fn normalize(mel_spectrogram_columns: &mut [f32], maximum_value: f32) {
    mel_spectrogram_columns
        .iter_mut()
        .for_each(|x| *x = ((*x).max(1e-10).log10().max(maximum_value - 8.0) + 4.0) / 4.0);
}
