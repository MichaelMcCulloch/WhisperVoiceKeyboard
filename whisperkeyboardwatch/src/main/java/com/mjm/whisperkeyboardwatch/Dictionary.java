package com.mjm.whisperkeyboardwatch;


import android.util.Log;

import androidx.annotation.NonNull;

import java.util.Map;

public class Dictionary {
    private final Vocab _vocab;

    private final Map<String, String> _phraseMap;

    public Dictionary(Vocab tokenMappings, Map<String, String> phraseMappings) {
        _vocab = tokenMappings;
        _phraseMap = phraseMappings;
    }

    /**
     * This method takes an int array 2D as an argument and returns a string that is composed of the words of the tokens in the array.
     *
     * @param output 2D int array of tokens
     * @return String composed of words of the tokens in the array
     */
    @NonNull
    public String tokensToString(int[][] output) {
        StringBuilder sb = new StringBuilder();
        for (int token : output[0]) {
            if (token == _vocab.token_eot) {
                break;
            }
            if (token != 50257 && token != 50362) {
                String word = _vocab.id_to_token.get(token);
                Log.i("tokenization", "token: " + token + " word " + word);
                sb.append(word);
            }
        }
        return sb.toString();
    }


    /**
     * This method takes a string as an argument and replaces key phrases with special tokens.
     *
     * @param text String to be injected with tokens
     * @return String with injected tokens
     */
    @NonNull
    public String injectTokens(String text) {
        String result = text;
        for (Map.Entry<String, String> entry : _phraseMap.entrySet()) {
            String phrase = entry.getKey();
            String token = entry.getValue();
            result = result.replace(phrase, token);
        }
        return result;
    }

}
