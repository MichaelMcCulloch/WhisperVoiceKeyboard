use std::{collections::HashMap, io::Read};

use anyhow::anyhow;
use ndk::asset::Asset;

use crate::{
    statics::WHISPER_VOCAB,
    whisper::{
        filters::Filters,
        vocab::{IsMultilingual, Vocab},
    },
};
pub(crate) fn init(mut buffer: Asset) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    extract_filters_and_vocab(&mut buffer).unwrap();
}
pub(crate) fn uninit() {
    unsafe {
        WHISPER_VOCAB.take();
    }
}

fn extract_filters_and_vocab(
    filters_vocab_gen_bin: &mut Asset,
) -> anyhow::Result<(Filters, Vocab)> {
    if read_u32(filters_vocab_gen_bin)? == 0x5553454e {
        let filter = extract_filters(filters_vocab_gen_bin)?;

        let vocab = extract_vocab(filters_vocab_gen_bin)?;

        Ok((filter, vocab))
    } else {
        Err(anyhow!("Bad Magic"))
    }
}

fn extract_filters(filters_vocab_gen_bin: &mut Asset) -> anyhow::Result<Filters> {
    let n_mel = read_i32(filters_vocab_gen_bin)?;
    let n_fft = read_i32(filters_vocab_gen_bin)?;
    assert_eq!(80, n_mel);
    assert_eq!(201, n_fft);
    let data = read_vec_f32(filters_vocab_gen_bin, (n_mel * n_fft) as usize)?;
    let filter = Filters::new(n_mel as usize, n_fft as usize, data);
    log::trace!(
        "Succeeded in Loading Filters!  {} Mels; {} Filters.",
        n_mel,
        n_fft
    );

    Ok(filter)
}

// TODO this smells pretty bad. but the compiler is happy so who am I to judge.
fn extract_vocab(filters_vocab_gen_bin: &mut Asset) -> anyhow::Result<Vocab> {
    let word_count = read_i32(filters_vocab_gen_bin)?;
    assert_eq!(50257, word_count);
    let mut words = HashMap::with_capacity(word_count as usize);
    for i in 0..word_count {
        let next_word_len = read_u32(filters_vocab_gen_bin)?;

        let word = read_string(filters_vocab_gen_bin, next_word_len).unwrap();
        words.entry(i).or_insert(word);
    }
    let mut vocab = Vocab::default();
    vocab.n_vocab = word_count;
    vocab.id_to_token = words;

    // Add some more vocab ids
    vocab.n_vocab = 51864;
    if vocab.is_multilingual() {
        vocab.token_eot += 1;
        vocab.token_sot += 1;
        vocab.token_prev += 1;
        vocab.token_solm += 1;
        vocab.token_not += 1;
        vocab.token_beg += 1;
    }
    for i in word_count..vocab.n_vocab {
        let word = if (i > vocab.token_beg) {
            format!("[_TT_{}]", i - vocab.token_beg)
        } else if i == vocab.token_eot {
            String::from("[_EOT_]")
        } else if i == vocab.token_sot {
            String::from("[_SOT_]")
        } else if i == vocab.token_prev {
            String::from("[_PREV_]")
        } else if i == vocab.token_not {
            String::from("[_NOT_]")
        } else if i == vocab.token_beg {
            String::from("[_BEG_]")
        } else {
            format!("[_extra_token_{}]", i)
        };
        vocab.id_to_token.entry(i).or_insert(word);
    }
    log::trace!(
        "Succeeded in Loading Vocab! {} ({}) Words.",
        vocab.n_vocab,
        vocab.id_to_token.len()
    );
    Ok(vocab)
}

fn read_vec_f32(asset: &mut Asset, number_of_bytes: usize) -> Result<Vec<f32>, anyhow::Error> {
    let mut data = vec![0u8; 4 * number_of_bytes];
    asset.read_exact(&mut data)?;
    let data: Vec<f32> = unsafe {
        // this may well be wrong. Although it's just a big block of bytes, not sure if it's le or be or what.
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
