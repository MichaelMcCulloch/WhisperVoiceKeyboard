use std::io::Read;

use anyhow::anyhow;
use ndk::asset::Asset;

use crate::consts::{N_FFT, N_MEL_BINS};

pub(crate) fn extract_filters(filters_vocab_gen_bin: &mut Asset) -> anyhow::Result<Vec<Vec<f32>>> {
    if read_u32(filters_vocab_gen_bin)? == 0x5553454e {
        let n_mel = read_i32(filters_vocab_gen_bin)?;
        let n_fft = read_i32(filters_vocab_gen_bin)?;
        assert_eq!(N_MEL_BINS as i32, n_mel);
        assert_eq!(N_FFT as i32, n_fft);
        let data = read_vec_of_vec_f32(filters_vocab_gen_bin, N_MEL_BINS, N_FFT)?;
        // Transpose the 2d array data

        Ok(data)
    } else {
        Err(anyhow!("Bad Magic"))
    }
}

fn read_vec_of_vec_f32(
    asset: &mut Asset,
    n_mel: usize,
    n_fft: usize,
) -> Result<Vec<Vec<f32>>, anyhow::Error> {
    let mut filters: Vec<Vec<f32>> = Vec::with_capacity(n_mel);
    for _i in 0..n_mel {
        let mut data = vec![0u8; 4 * n_fft];
        asset.read_exact(&mut data)?;

        unsafe {
            let (_pre, floats, _post) = data.as_slice().align_to::<f32>();
            assert!(_pre.is_empty() && _post.is_empty());
            filters.push(floats.to_vec());
        };
    }

    Ok(filters)
}

fn read_u32(asset: &mut Asset) -> anyhow::Result<u32> {
    let mut buffer = [0u8; 4];
    asset.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_i32(asset: &mut Asset) -> anyhow::Result<i32> {
    let mut buffer = [0u8; 4];
    asset.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}
