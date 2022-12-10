pub(crate) struct Filters {
    pub n_mel: usize,
    pub n_fft: usize,
    pub data: Vec<f32>,
}
impl Filters {
    pub(crate) fn new(n_mel: usize, n_fft: usize, data: Vec<f32>) -> Filters {
        Self { n_mel, n_fft, data }
    }
}
