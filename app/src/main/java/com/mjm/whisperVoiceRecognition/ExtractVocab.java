package com.mjm.whisperVoiceRecognition;

import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class ExtractVocab {

    public static Vocab extractVocab(InputStream filters_vocab_gen_bin) throws IOException {
        if (readI32(filters_vocab_gen_bin) == 0x5553454e) {


            readI32(filters_vocab_gen_bin);
            readI32(filters_vocab_gen_bin);
            readVecF32(filters_vocab_gen_bin, 80 * 201);

            Vocab vocab = new Vocab();
            int word_count = readI32(filters_vocab_gen_bin);
            assert (50257 == word_count);
            HashMap<Integer, String> words = new HashMap<>(word_count);
            for (int i = 0; i < word_count; i++) {
                int nextWordLen = readU32(filters_vocab_gen_bin);
                String word = readString(filters_vocab_gen_bin, nextWordLen);
                words.put(i, word);
            }
            vocab.n_vocab = word_count;
            vocab.id_to_token = words;
            if (vocab.isMultilingual()) {
                vocab.token_eot += 1;
                vocab.token_sot += 1;
                vocab.token_prev += 1;
                vocab.token_solm += 1;
                vocab.token_not += 1;
                vocab.token_beg += 1;
            }
            for (int i = word_count; i < vocab.n_vocab; i++) {
                String word;
                if (i > vocab.token_beg) {
                    word = "[_TT_" + (i - vocab.token_beg) + "]";
                } else if (i == vocab.token_eot) {
                    word = "[_EOT_]";
                } else if (i == vocab.token_sot) {
                    word = "[_SOT_]";
                } else if (i == vocab.token_prev) {
                    word = "[_PREV_]";
                } else if (i == vocab.token_not) {
                    word = "[_NOT_]";
                } else if (i == vocab.token_beg) {
                    word = "[_BEG_]";
                } else {
                    word = "[_extra_token_" + i + "]";
                }
                vocab.id_to_token.put(i, word);
            }
            System.out.println("Succeeded in Loading Vocab! " + vocab.n_vocab + " (" + vocab.id_to_token.size() + ") Words.");
            return vocab;
        } else throw new IOException("bad magic");
    }

    public static List<Float> readVecF32(InputStream asset, int numberOfBytes) throws IOException {
        byte[] data = new byte[4 * numberOfBytes];
        asset.read(data);

        List<Float> vec = new LinkedList<Float>();
        for (int i = 0; i < data.length; i += 4) {
            byte[] chunk = new byte[4];
            System.arraycopy(data, i, chunk, 0, 4);
            float f = ByteBuffer.wrap(chunk).order(ByteOrder.LITTLE_ENDIAN).getFloat();
            vec.add(f);
        }
        return vec;
    }

    public static String readString(InputStream asset, int stringLen) throws IOException {
        byte[] data = new byte[stringLen];
        asset.read(data);

        StringBuilder sb = new StringBuilder();
        for (byte b : data) {
            sb.append((char) b);
        }
        return sb.toString();
    }

    public static int readI32(InputStream asset) throws IOException {
        byte[] buffer = new byte[4];
        asset.read(buffer);
        int anInt = ByteBuffer.wrap(buffer).order(ByteOrder.LITTLE_ENDIAN).getInt();
        return anInt;
    }

    public static int readU32(InputStream asset) throws IOException {
        byte[] buffer = new byte[4];
        asset.read(buffer);
        int anInt = ByteBuffer.wrap(buffer).order(ByteOrder.LITTLE_ENDIAN).getInt();
        return anInt;
    }

}
