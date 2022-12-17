use std::{collections::HashMap, io::Read};

use anyhow::anyhow;
use ndk::asset::Asset;

use crate::consts::{N_FFT, N_MEL_BINS};

pub(crate) fn extract_filters_and_vocab(
    filters_vocab_gen_bin: &mut Asset,
) -> anyhow::Result<Vec<f32>> {
    if read_u32(filters_vocab_gen_bin)? == 0x5553454e {
        let filter = extract_filters(filters_vocab_gen_bin)?;

        // let vocab = extract_vocab(filters_vocab_gen_bin)?;

        Ok(filter)
    } else {
        Err(anyhow!("Bad Magic"))
    }
}

fn extract_filters(filters_vocab_gen_bin: &mut Asset) -> anyhow::Result<Vec<f32>> {
    let n_mel = read_i32(filters_vocab_gen_bin)?;
    let n_fft = read_i32(filters_vocab_gen_bin)?;
    assert_eq!(N_MEL_BINS as i32, n_mel);
    assert_eq!(N_FFT as i32, n_fft);
    let data = read_vec_f32(filters_vocab_gen_bin, (n_mel * n_fft) as usize)?;
    // Transpose the 2d array data

    Ok(data)
}

fn read_vec_f32(asset: &mut Asset, number_of_bytes: usize) -> Result<Vec<f32>, anyhow::Error> {
    let mut data = vec![0u8; 4 * number_of_bytes];
    asset.read_exact(&mut data)?;

    let data: Vec<f32> = unsafe {
        let (_, floats, _) = data.as_slice().align_to::<f32>();
        floats.to_vec()
    };

    Ok(data)
}

fn read_u32(asset: &mut Asset) -> anyhow::Result<u32> {
    let mut buffer = [0u8; 4];
    asset.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_string(asset: &mut Asset, string_len: u32) -> anyhow::Result<String> {
    let mut data = vec![0u8; string_len as usize];
    asset.read_exact(&mut data)?;

    let string = data.iter().map(|b| *b as char).collect();
    Ok(string)
}

fn read_i32(asset: &mut Asset) -> anyhow::Result<i32> {
    let mut buffer = [0u8; 4];
    asset.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}
