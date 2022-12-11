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
///The log_mel_spectrogram function computes the log-mel spectrogram of an audio signal. The input to the function is a slice of f32 values representing the audio signal in little-endian format. The function returns a Mel value, which is a type representing the log-mel spectrogram of the audio signal.
///
///This function uses the Fast Fourier Transform (FFT) to compute the spectrogram of the audio signal, and then applies a Mel filterbank to the spectrogram to compute the log-mel spectrogram. The Mel filterbank is defined by the WHISPER_FILTERS constant, which is a static array of floating-point values representing the weights of the filters in the filterbank.
///
///The function applies a Hann window to the audio signal before computing the FFT, in order to reduce spectral leakage. The size of the FFT and the hop length of the window are determined by the WHISPER_N_FFT and WHISPER_HOP_LENGTH constants, respectively. The function uses a platform-specific implementation of the FFT, specified by the target_arch attribute of the function. The function is only implemented for the x86_64 and aarch64 architectures.
pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Vec<f32> {
    // Create a new FFT planner
    #[cfg(target_arch = "x86_64")]
    let mut planner = FftPlanner::new();
    #[cfg(target_arch = "aarch64")]
    let mut planner = FftPlannerNeon::new().unwrap();

    // Create an FFT object with the specified window size
    let fft = planner.plan_fft_forward(N_FFT);

    // Set up the Hann window
    let window: Vec<f32> = (0..N_FFT)
        .map(|i| HANN_ALPHA * (1.0 - (HANN_BETA * i as f32 / N_FFT as f32).cos()))
        .collect();

    // Create a buffer to hold the overlapping audio frames
    let mut buffer: Vec<f32> = vec![0.0; N_FFT];

    let mut columns = Vec::with_capacity(MEL_LEN * N_MEL);

    match unsafe { WHISPER_FILTERS.take() } {
        Some(filters) => {
            // Iterate through the audio stream, computing the STFT for each frame of audio
            for frame in f32le_audio.chunks(HOP_LENGTH) {
                // Overlap-add the current audio frame onto the buffer
                for (i, sample) in frame.iter().enumerate() {
                    buffer[i] += sample;
                }

                // Apply the Hann window to the buffer
                for (i, sample) in buffer.iter_mut().enumerate() {
                    *sample *= window[i];
                }

                // Convert the samples in the buffer to complex numbers
                let mut input: Vec<Complex32> =
                    buffer.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();

                // Create a buffer to hold the STFT output
                let mut output: Vec<Complex32> = vec![Complex { re: 0.0, im: 0.0 }; N_FFT];

                // Compute the FFT of the audio frame
                let mut scratch = vec![Complex::zero(); fft.get_outofplace_scratch_len()];
                fft.process_outofplace_with_scratch(&mut input[..], &mut output[..], &mut scratch);

                // Calculate the power spectrum by squaring the real and imaginary parts of the FFT output
                let mut power_spectrum = vec![0.0; N_FFT as usize];
                for i in 0..N_FFT as usize {
                    power_spectrum[i] = output[i].norm();
                }

                // Sum the power spectrum with its mirrored counterpart
                for i in 1..N_FFT as usize / 2 {
                    power_spectrum[i] += power_spectrum[N_FFT as usize - i];
                }
                // Apply the Mel-frequency filters to the summed power spectrum

                let mut log_mel_spec = vec![0.0; N_MEL as usize];
                for i in 0..N_MEL as usize {
                    for j in 0..N_FFT {
                        log_mel_spec[i] += filters.data[i * N_FFT + j] * power_spectrum[j];
                    }
                }

                columns.extend(log_mel_spec);

                // Clear the buffer for the next frame of samples
                buffer.copy_from_slice(&[0.0; N_FFT]);
            }
            // Clamp and normalize
            let mmax = columns.iter().fold(f32::MIN, |acc, f| f.max(acc));
            for x in &mut columns {
                *x = (*x).max(1e-10).log10().min(mmax - 8.0);
                *x = (*x + 4.0) / 4.0;
            }

            // Return the log Mel-frequency spectrogram
            unsafe { WHISPER_FILTERS.replace(filters) };
        }
        None => todo!(),
    }

    columns
}
