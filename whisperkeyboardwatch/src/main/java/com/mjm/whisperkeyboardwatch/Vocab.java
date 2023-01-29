package com.mjm.whisperkeyboardwatch;

import java.util.HashMap;

public class Vocab {

    public int n_vocab;
    public int token_eot;
    public int token_sot;
    public int token_prev;
    public int token_solm;
    public int token_not;
    public int token_beg;
    public HashMap<Integer, String> id_to_token;

    public Vocab() {
        // Magic Numbers evidently derived from https://github.com/ggerganov/whisper.cpp
        this.n_vocab = 51864;
        this.token_eot = 50256;
        this.token_sot = 50257;
        this.token_prev = 50360;
        this.token_solm = 50361;
        this.token_not = 50362;
        this.token_beg = 50363;
        this.id_to_token = new HashMap<Integer, String>();
    }

    public boolean isMultilingual() {
        return this.n_vocab == 51865;
    }

}
