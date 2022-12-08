pub(crate) struct Filters<'filters> {
    n_mel: usize,
    n_fft: usize,
    data: &'filters [f32],
}
