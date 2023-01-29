use std::sync::Arc;

use crate::{
    consts::{FFT_LEN, MEL_LEN, N_FFT, N_MEL_BINS},
    mel::compute_mel,
    statics::WHISPER_FILTERS,
    work_buffer::populate_working_buffers,
};

use rustfft::{num_complex::Complex32, Fft};

/// This method is used to generate a log mel spectrogram from a given `f32le_audio` vector. It does this by applying a window function and using an FFT process to compute the power spectrum, before computing the logmel spectrogram.
///
/// The spectrogram is also normalized to have a maximum value of `1.0`.
///
/// ## Arguments
/// - `f32le_audio`: A vector of floats of audio data in little endian format.
///
/// ## Return Value
/// A vector of floats representing the log mel spectrogram.
// Publishes a function called log_mel_spectrogram which takes a array of f32 numbers as an argument
pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Vec<f32> {
    match unsafe { WHISPER_FILTERS.take() } {
        Some(filters) => {
            let fft_process = get_fft_plan(FFT_LEN);

            let mut power_spectrum_columns = vec![vec![0.0; N_FFT]; MEL_LEN];

            let hann = hann_window(FFT_LEN);

            let mut working_buffer = populate_working_buffers(hann, f32le_audio);

            for i in 0..MEL_LEN {
                fft_process.process(&mut working_buffer[i]);

                let power_spectrum = compute_power(&working_buffer[i], N_FFT);

                power_spectrum_columns[i].copy_from_slice(&power_spectrum);
            }

            let log_mel_spectrogram =
                compute_mel(&power_spectrum_columns, &filters, MEL_LEN, N_MEL_BINS);

            unsafe { WHISPER_FILTERS.replace(filters) };

            let mut lms = log_mel_spectrogram
                .into_iter()
                .flat_map(|f| f)
                .collect::<Vec<_>>();
            normalize(&mut lms);

            lms
        }

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
    #[cfg(not(target_arch = "aarch64"))]
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

/// Compute the power spectrum from an FFT result buffer.
fn compute_power(fft_work_buffer: &[Complex32], n_fft: usize) -> Vec<f32> {
    let mut power_spectrum = vec![0.0; fft_work_buffer.len()];

    for i in 0..fft_work_buffer.len() {
        power_spectrum[i] = fft_work_buffer[i].norm_sqr()
    }

    // Perform doubling of the power spectrum
    for j in 1..n_fft {
        power_spectrum[j] += power_spectrum[fft_work_buffer.len() - j]
    }

    power_spectrum[0..n_fft].to_vec()
}

/// Normalize the mel spectrogram columns buffer.
fn normalize(mel_spectrogram_columns: &mut [f32]) {
    // Compute the maximum value of the mel spectrogram columns buffer.
    mel_spectrogram_columns
        .iter_mut()
        .for_each(|x| *x = (*x).max(1e-10).log10());

    let maximum_value = mel_spectrogram_columns
        .iter()
        .fold(-1e20f32, |acc, f| f.max(acc));

    mel_spectrogram_columns
        .iter_mut()
        .for_each(|x| *x = ((*x).max(maximum_value - 8.0) + 4.0) / 4.0);
}
