#[derive(Debug)]
pub(crate) struct WhisperFilters {
    n_mel: i32,
    n_fft: i32,
    data: Vec<f32>,
}

impl WhisperFilters {
    pub(crate) fn new(n_mel: i32, n_fft: i32, data: Vec<f32>) -> WhisperFilters {
        Self { n_mel, n_fft, data }
    }
}
