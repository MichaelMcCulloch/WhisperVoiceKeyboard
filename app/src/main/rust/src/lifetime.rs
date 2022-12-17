use crate::{consts::*, extract, statics::WHISPER_FILTERS};
use ndk::asset::Asset;

pub(crate) fn init(mut buffer: Asset) {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));

    match unsafe { WHISPER_FILTERS.take() } {
        None => {
            let filter = extract::extract_filters_and_vocab(&mut buffer).unwrap();
            let mut f = vec![vec![0.0; N_FFT]; N_MEL_BINS];
            for i in 0..N_MEL_BINS {
                f[i].copy_from_slice(&filter[i * N_FFT..(i + 1) * N_FFT])
            }

            log::info!("READ FILTERS");

            unsafe { WHISPER_FILTERS.replace(transpose(f)) };
        }
        _ => {
            log::info!("Library already inititalized");
            unimplemented!()
        }
    }
}
pub(crate) fn uninit() {
    unsafe {
        WHISPER_FILTERS.take();
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
