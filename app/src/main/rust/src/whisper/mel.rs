pub(crate) struct Mel<'mel> {
    n_len: usize,
    n_mel: usize,
    data: &'mel [f32],
}
