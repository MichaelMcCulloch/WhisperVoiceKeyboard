pub(crate) struct Mel {
    n_len: usize,
    n_mel: usize,
    pub data: Vec<f32>,
}

impl Mel {
    pub(crate) fn new(n_len: usize, n_mel: usize, data: Vec<f32>) -> Self {
        Mel { n_len, n_mel, data }
    }
}
