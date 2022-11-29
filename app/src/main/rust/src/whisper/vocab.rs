use std::collections::HashMap;

pub(crate) struct WhisperVocab {
    n_vocab: i32,
    token_eot: i32,
    token_sot: i32,
    token_prev: i32,
    token_solm: i32,
    token_not: i32,
    token_beg: i32,
    id_to_token: HashMap<i32, String>,
}

impl Default for WhisperVocab {
    fn default() -> Self {
        // Magic Numbers evidently derived from https://github.com/ggerganov/whisper.cpp
        Self {
            n_vocab: 51864,
            token_eot: 50256,
            token_sot: 50257,
            token_prev: 50360,
            token_solm: 50361,
            token_not: 50362,
            token_beg: 50363,
            id_to_token: HashMap::new(),
        }
    }
}

pub(crate) trait IsMultilingual {
    fn is_multilingual(&self) -> bool;
}

impl IsMultilingual for WhisperVocab {
    fn is_multilingual(&self) -> bool {
        self.n_vocab == 51865
    }
}
