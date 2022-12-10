use crate::{
    consts::{
        WHISPER_CHUNK_SIZE, WHISPER_HOP_LENGTH, WHISPER_MEL_LEN, WHISPER_N_FFT, WHISPER_N_MEL,
        WHISPER_SAMPLE_RATE,
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
use rustfft::FftPlanner;
use rustfft::{num_complex::Complex32, num_traits::Zero};
const HANN_ALPHA: f32 = 0.5;
const HANN_BETA: f32 = -2.0 * std::f32::consts::PI;

///The log_mel_spectrogram function computes the log-mel spectrogram of an audio signal. The input to the function is a slice of f32 values representing the audio signal in little-endian format. The function returns a Mel value, which is a type representing the log-mel spectrogram of the audio signal.
///
///This function uses the Fast Fourier Transform (FFT) to compute the spectrogram of the audio signal, and then applies a Mel filterbank to the spectrogram to compute the log-mel spectrogram. The Mel filterbank is defined by the WHISPER_FILTERS constant, which is a static array of floating-point values representing the weights of the filters in the filterbank.
///
///The function applies a Hann window to the audio signal before computing the FFT, in order to reduce spectral leakage. The size of the FFT and the hop length of the window are determined by the WHISPER_N_FFT and WHISPER_HOP_LENGTH constants, respectively. The function uses a platform-specific implementation of the FFT, specified by the target_arch attribute of the function. The function is only implemented for the x86_64 and aarch64 architectures.
pub(crate) fn log_mel_spectrogram(f32le_audio: &[f32]) -> Mel {
    // Create a new FFT planner
    #[cfg(target_arch = "x86_64")]
    let mut planner = FftPlanner::new();
    #[cfg(target_arch = "aarch64")]
    let mut planner = FftPlanner::new();

    // Create an FFT object with the specified window size
    let fft = planner.plan_fft_forward(WHISPER_N_FFT);

    // Set up the Hann window
    let window: Vec<f32> = (0..WHISPER_N_FFT)
        .map(|i| HANN_ALPHA * (1.0 - (HANN_BETA * i as f32 / WHISPER_N_FFT as f32).cos()))
        .collect();

    // Create a buffer to hold the overlapping audio frames
    let mut buffer: Vec<f32> = vec![0.0; WHISPER_N_FFT];

    let mut columns = Vec::with_capacity(3000);

    match unsafe { WHISPER_FILTERS.take() } {
        Some(filters) => {
            let filt_2d = Array2::from_shape_fn((201, filters.data.len() / 201), |(i, j)| {
                filters.data[i + 2 * j]
            });
            // Iterate through the audio stream, computing the STFT for each frame of audio
            for frame in f32le_audio.chunks(WHISPER_HOP_LENGTH) {
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
                let mut output: Vec<Complex32> = vec![Complex { re: 0.0, im: 0.0 }; WHISPER_N_FFT];

                // Compute the STFT of the audio framefn process(&self, buffer: &mut [Complex<T>]) {
                let mut scratch = vec![Complex::zero(); fft.get_outofplace_scratch_len()];
                fft.process_outofplace_with_scratch(&mut input[..], &mut output[..], &mut scratch);
                // Compute the power spectrum of the audio frame

                let stft = Array2::from_shape_vec((2, output.len() / 2), output).unwrap();
                let power_spectrum = stft.map(|x| x.norm() * x.norm());

                // Compute the log-mel spectogram by convolving the STFT with the Mel filterbank
                let log_mel_spectrogram = convolve(
                    &power_spectrum,
                    &filt_2d,
                    ndarray_ndimage::BorderMode::Nearest,
                    0,
                );
                columns.push(log_mel_spectrogram);
                // Shift the samples in the buffer to prepare for the next frame
                for i in 0..WHISPER_HOP_LENGTH {
                    buffer[i] = buffer[i + WHISPER_HOP_LENGTH];
                }
                for i in WHISPER_HOP_LENGTH..WHISPER_N_FFT {
                    buffer[i] = 0.0;
                }
            }

            unsafe { WHISPER_FILTERS.replace(filters) };
        }
        None => todo!(),
    }
    log::info!("t");
    let data = columns.into_iter().flat_map(|e| e).collect::<Vec<_>>();
    let n_mel =
        WHISPER_SAMPLE_RATE as usize * WHISPER_CHUNK_SIZE as usize / WHISPER_HOP_LENGTH as usize;

    assert!(data.len() == n_mel * WHISPER_N_FFT);
    let m = Mel::new(WHISPER_MEL_LEN as usize, n_mel, data);
    m
}
