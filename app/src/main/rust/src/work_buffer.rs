use nalgebra::Complex;
use rustfft::{num_complex::Complex32, num_traits::Zero};

use crate::consts::*;
#[cfg(not(target_arch = "aarch64"))]
pub(crate) fn populate_working_buffers(hann: Vec<f32>, f32le_audio: &[f32]) -> Vec<Vec<Complex32>> {
    let mut working_buffer = vec![vec![Complex::zero(); FFT_LEN]; MEL_LEN];
    for i in 0..MEL_LEN {
        let offset = i * HOP_LENGTH;
        for j in 0..FFT_LEN {
            if offset + j < SAMPLE_RATE * RECORDING_LEN {
                working_buffer[i][j] = Complex {
                    re: hann[j] * f32le_audio[offset + j],
                    im: 0.0,
                };
            } else {
                working_buffer[i][j] = Complex::zero();
            }
        }
    }
    working_buffer
}
#[cfg(target_arch = "aarch64")]
pub(crate) fn populate_working_buffers(han: Vec<f32>, aud: &[f32]) -> Vec<Vec<Complex32>> {
    use std::arch::aarch64::{vdupq_n_f32, vfmaq_f32, vgetq_lane_f32, vld1q_f32_x4};

    let mut wrk = vec![vec![Complex::zero(); FFT_LEN]; MEL_LEN];

    unsafe {
        let han_0 = vld1q_f32_x4(han[0..16].as_ptr());
        let han_16 = vld1q_f32_x4(han[16..32].as_ptr());
        let han_32 = vld1q_f32_x4(han[32..48].as_ptr());
        let han_48 = vld1q_f32_x4(han[48..64].as_ptr());
        let han_64 = vld1q_f32_x4(han[64..80].as_ptr());
        let han_80 = vld1q_f32_x4(han[80..96].as_ptr());
        let han_96 = vld1q_f32_x4(han[96..112].as_ptr());
        let han_112 = vld1q_f32_x4(han[112..128].as_ptr());
        let han_128 = vld1q_f32_x4(han[128..144].as_ptr());
        let han_144 = vld1q_f32_x4(han[144..160].as_ptr());
        let han_160 = vld1q_f32_x4(han[160..176].as_ptr());
        let han_176 = vld1q_f32_x4(han[176..192].as_ptr());
        let han_192 = vld1q_f32_x4(han[192..208].as_ptr());
        let han_208 = vld1q_f32_x4(han[208..224].as_ptr());
        let han_224 = vld1q_f32_x4(han[224..240].as_ptr());
        let han_240 = vld1q_f32_x4(han[240..256].as_ptr());
        let han_256 = vld1q_f32_x4(han[256..272].as_ptr());
        let han_272 = vld1q_f32_x4(han[272..288].as_ptr());
        let han_288 = vld1q_f32_x4(han[288..304].as_ptr());
        let han_304 = vld1q_f32_x4(han[304..320].as_ptr());
        let han_320 = vld1q_f32_x4(han[320..336].as_ptr());
        let han_336 = vld1q_f32_x4(han[336..352].as_ptr());
        let han_352 = vld1q_f32_x4(han[352..368].as_ptr());
        let han_368 = vld1q_f32_x4(han[368..384].as_ptr());
        let han_384 = vld1q_f32_x4(han[384..400].as_ptr());
        for i in 0..MEL_LEN - 2 {
            let offset = i * HOP_LENGTH;
            let aud_0 = vld1q_f32_x4(aud[(offset + 0)..(offset + 16)].as_ptr());
            let aud_16 = vld1q_f32_x4(aud[(offset + 16)..(offset + 32)].as_ptr());
            let aud_32 = vld1q_f32_x4(aud[(offset + 32)..(offset + 48)].as_ptr());
            let aud_48 = vld1q_f32_x4(aud[(offset + 48)..(offset + 64)].as_ptr());
            let aud_64 = vld1q_f32_x4(aud[(offset + 64)..(offset + 80)].as_ptr());
            let aud_80 = vld1q_f32_x4(aud[(offset + 80)..(offset + 96)].as_ptr());
            let aud_96 = vld1q_f32_x4(aud[(offset + 96)..(offset + 112)].as_ptr());
            let aud_112 = vld1q_f32_x4(aud[(offset + 112)..(offset + 128)].as_ptr());
            let aud_128 = vld1q_f32_x4(aud[(offset + 128)..(offset + 144)].as_ptr());
            let aud_144 = vld1q_f32_x4(aud[(offset + 144)..(offset + 160)].as_ptr());
            let aud_160 = vld1q_f32_x4(aud[(offset + 160)..(offset + 176)].as_ptr());
            let aud_176 = vld1q_f32_x4(aud[(offset + 176)..(offset + 192)].as_ptr());
            let aud_192 = vld1q_f32_x4(aud[(offset + 192)..(offset + 208)].as_ptr());
            let aud_208 = vld1q_f32_x4(aud[(offset + 208)..(offset + 224)].as_ptr());
            let aud_224 = vld1q_f32_x4(aud[(offset + 224)..(offset + 240)].as_ptr());
            let aud_240 = vld1q_f32_x4(aud[(offset + 240)..(offset + 256)].as_ptr());
            let aud_256 = vld1q_f32_x4(aud[(offset + 256)..(offset + 272)].as_ptr());
            let aud_272 = vld1q_f32_x4(aud[(offset + 272)..(offset + 288)].as_ptr());
            let aud_288 = vld1q_f32_x4(aud[(offset + 288)..(offset + 304)].as_ptr());
            let aud_304 = vld1q_f32_x4(aud[(offset + 304)..(offset + 320)].as_ptr());
            let aud_320 = vld1q_f32_x4(aud[(offset + 320)..(offset + 336)].as_ptr());
            let aud_336 = vld1q_f32_x4(aud[(offset + 336)..(offset + 352)].as_ptr());
            let aud_352 = vld1q_f32_x4(aud[(offset + 352)..(offset + 368)].as_ptr());
            let aud_368 = vld1q_f32_x4(aud[(offset + 368)..(offset + 384)].as_ptr());
            let aud_384 = vld1q_f32_x4(aud[(offset + 384)..(offset + 400)].as_ptr());

            let res_0_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_0.0, aud_0.0);
            let res_0_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_0.1, aud_0.1);
            let res_0_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_0.2, aud_0.2);
            let res_0_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_0.3, aud_0.3);
            let res_16_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_16.0, aud_16.0);
            let res_16_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_16.1, aud_16.1);
            let res_16_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_16.2, aud_16.2);
            let res_16_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_16.3, aud_16.3);
            let res_32_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_32.0, aud_32.0);
            let res_32_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_32.1, aud_32.1);
            let res_32_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_32.2, aud_32.2);
            let res_32_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_32.3, aud_32.3);
            let res_48_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_48.0, aud_48.0);
            let res_48_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_48.1, aud_48.1);
            let res_48_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_48.2, aud_48.2);
            let res_48_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_48.3, aud_48.3);
            let res_64_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_64.0, aud_64.0);
            let res_64_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_64.1, aud_64.1);
            let res_64_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_64.2, aud_64.2);
            let res_64_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_64.3, aud_64.3);
            let res_80_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_80.0, aud_80.0);
            let res_80_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_80.1, aud_80.1);
            let res_80_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_80.2, aud_80.2);
            let res_80_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_80.3, aud_80.3);
            let res_96_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_96.0, aud_96.0);
            let res_96_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_96.1, aud_96.1);
            let res_96_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_96.2, aud_96.2);
            let res_96_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_96.3, aud_96.3);
            let res_112_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_112.0, aud_112.0);
            let res_112_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_112.1, aud_112.1);
            let res_112_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_112.2, aud_112.2);
            let res_112_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_112.3, aud_112.3);
            let res_128_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_128.0, aud_128.0);
            let res_128_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_128.1, aud_128.1);
            let res_128_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_128.2, aud_128.2);
            let res_128_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_128.3, aud_128.3);
            let res_144_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_144.0, aud_144.0);
            let res_144_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_144.1, aud_144.1);
            let res_144_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_144.2, aud_144.2);
            let res_144_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_144.3, aud_144.3);
            let res_160_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_160.0, aud_160.0);
            let res_160_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_160.1, aud_160.1);
            let res_160_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_160.2, aud_160.2);
            let res_160_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_160.3, aud_160.3);
            let res_176_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_176.0, aud_176.0);
            let res_176_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_176.1, aud_176.1);
            let res_176_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_176.2, aud_176.2);
            let res_176_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_176.3, aud_176.3);
            let res_192_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_192.0, aud_192.0);
            let res_192_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_192.1, aud_192.1);
            let res_192_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_192.2, aud_192.2);
            let res_192_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_192.3, aud_192.3);
            let res_208_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_208.0, aud_208.0);
            let res_208_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_208.1, aud_208.1);
            let res_208_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_208.2, aud_208.2);
            let res_208_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_208.3, aud_208.3);
            let res_224_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_224.0, aud_224.0);
            let res_224_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_224.1, aud_224.1);
            let res_224_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_224.2, aud_224.2);
            let res_224_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_224.3, aud_224.3);
            let res_240_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_240.0, aud_240.0);
            let res_240_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_240.1, aud_240.1);
            let res_240_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_240.2, aud_240.2);
            let res_240_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_240.3, aud_240.3);
            let res_256_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_256.0, aud_256.0);
            let res_256_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_256.1, aud_256.1);
            let res_256_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_256.2, aud_256.2);
            let res_256_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_256.3, aud_256.3);
            let res_272_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_272.0, aud_272.0);
            let res_272_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_272.1, aud_272.1);
            let res_272_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_272.2, aud_272.2);
            let res_272_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_272.3, aud_272.3);
            let res_288_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_288.0, aud_288.0);
            let res_288_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_288.1, aud_288.1);
            let res_288_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_288.2, aud_288.2);
            let res_288_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_288.3, aud_288.3);
            let res_304_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_304.0, aud_304.0);
            let res_304_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_304.1, aud_304.1);
            let res_304_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_304.2, aud_304.2);
            let res_304_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_304.3, aud_304.3);
            let res_320_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_320.0, aud_320.0);
            let res_320_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_320.1, aud_320.1);
            let res_320_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_320.2, aud_320.2);
            let res_320_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_320.3, aud_320.3);
            let res_336_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_336.0, aud_336.0);
            let res_336_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_336.1, aud_336.1);
            let res_336_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_336.2, aud_336.2);
            let res_336_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_336.3, aud_336.3);
            let res_352_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_352.0, aud_352.0);
            let res_352_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_352.1, aud_352.1);
            let res_352_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_352.2, aud_352.2);
            let res_352_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_352.3, aud_352.3);
            let res_368_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_368.0, aud_368.0);
            let res_368_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_368.1, aud_368.1);
            let res_368_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_368.2, aud_368.2);
            let res_368_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_368.3, aud_368.3);
            let res_384_0 = vfmaq_f32(vdupq_n_f32(0.0f32), han_384.0, aud_384.0);
            let res_384_1 = vfmaq_f32(vdupq_n_f32(0.0f32), han_384.1, aud_384.1);
            let res_384_2 = vfmaq_f32(vdupq_n_f32(0.0f32), han_384.2, aud_384.2);
            let res_384_3 = vfmaq_f32(vdupq_n_f32(0.0f32), han_384.3, aud_384.3);

            wrk[i].copy_from_slice(&[
                Complex {
                    re: vgetq_lane_f32(res_0_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_0_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_16_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_32_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_48_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_64_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_80_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_96_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_112_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_128_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_144_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_160_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_176_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_192_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_208_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_224_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_240_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_256_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_272_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_288_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_304_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_320_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_336_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_352_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_368_3, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_0, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_0, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_0, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_0, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_1, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_1, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_1, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_1, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_2, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_2, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_2, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_2, 3),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_3, 0),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_3, 1),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_3, 2),
                    im: 0.0,
                },
                Complex {
                    re: vgetq_lane_f32(res_384_3, 3),
                    im: 0.0,
                },
            ])
        }
    } // the last few moments of audio may be fucked.
    wrk[MEL_LEN - 2].copy_from_slice(&[Complex { re: 0.0, im: 0.0 }; 400]);
    wrk[MEL_LEN - 1].copy_from_slice(&[Complex { re: 0.0, im: 0.0 }; 400]);
    // for j in 0..320 {
    //     wrk[MEL_LEN - 2][j] = Complex {
    //         re: han[j] * aud[(MEL_LEN - 2) * HOP_LENGTH + j],
    //         im: 0.0,
    //     };
    // }
    // for j in 0..160 {
    //     wrk[MEL_LEN - 1][j] = Complex {
    //         re: han[j] * aud[(MEL_LEN - 1) * HOP_LENGTH + j],
    //         im: 0.0,
    //     };
    // }
    wrk
}
