pub(crate) struct WhisperMel {
    n_len: i32,
    n_mel: i32,
    data: Vec<f32>,
}
impl WhisperMel {
    pub(crate) fn new(n_len: i32, n_mel: i32, data: Vec<f32>) -> WhisperMel {
        Self { n_len, n_mel, data }
    }
}
