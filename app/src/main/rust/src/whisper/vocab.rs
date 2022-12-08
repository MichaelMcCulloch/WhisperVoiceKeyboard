use std::collections::HashMap;

pub(crate) struct Vocab {
    pub n_vocab: i32,
    pub token_eot: i32,
    pub token_sot: i32,
    pub token_prev: i32,
    pub token_solm: i32,
    pub token_not: i32,
    pub token_beg: i32,
    pub id_to_token: HashMap<i32, String>,
}

impl Default for Vocab {
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

impl IsMultilingual for Vocab {
    fn is_multilingual(&self) -> bool {
        self.n_vocab == 51865
    }
}
