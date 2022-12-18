use crate::consts::N_FFT;

/// Compute the log mel spectrogram from a power spectrum buffer and filters.
#[cfg(not(target_arch = "aarch64"))]
pub(crate) fn compute_mel(
    power_spectrum: &[Vec<f32>],
    filters: &[Vec<f32>],
    n_mel_frames: usize,
    mel_bins: usize,
) -> Vec<Vec<f32>> {
    let mut spectrogram = vec![vec![0.0; mel_bins]; n_mel_frames];
    for i in 0..n_mel_frames {
        for j in 0..mel_bins {
            spectrogram[i][j] += power_spectrum[i][0] * filters[j][0];
            spectrogram[i][j] += power_spectrum[i][1] * filters[j][1];
            spectrogram[i][j] += power_spectrum[i][2] * filters[j][2];
            spectrogram[i][j] += power_spectrum[i][3] * filters[j][3];
            spectrogram[i][j] += power_spectrum[i][4] * filters[j][4];
            spectrogram[i][j] += power_spectrum[i][5] * filters[j][5];
            spectrogram[i][j] += power_spectrum[i][6] * filters[j][6];
            spectrogram[i][j] += power_spectrum[i][7] * filters[j][7];
            spectrogram[i][j] += power_spectrum[i][8] * filters[j][8];
            spectrogram[i][j] += power_spectrum[i][9] * filters[j][9];
            spectrogram[i][j] += power_spectrum[i][10] * filters[j][10];
            spectrogram[i][j] += power_spectrum[i][11] * filters[j][11];
            spectrogram[i][j] += power_spectrum[i][12] * filters[j][12];
            spectrogram[i][j] += power_spectrum[i][13] * filters[j][13];
            spectrogram[i][j] += power_spectrum[i][14] * filters[j][14];
            spectrogram[i][j] += power_spectrum[i][15] * filters[j][15];
            spectrogram[i][j] += power_spectrum[i][16] * filters[j][16];
            spectrogram[i][j] += power_spectrum[i][17] * filters[j][17];
            spectrogram[i][j] += power_spectrum[i][18] * filters[j][18];
            spectrogram[i][j] += power_spectrum[i][19] * filters[j][19];
            spectrogram[i][j] += power_spectrum[i][20] * filters[j][20];
            spectrogram[i][j] += power_spectrum[i][21] * filters[j][21];
            spectrogram[i][j] += power_spectrum[i][22] * filters[j][22];
            spectrogram[i][j] += power_spectrum[i][23] * filters[j][23];
            spectrogram[i][j] += power_spectrum[i][24] * filters[j][24];
            spectrogram[i][j] += power_spectrum[i][25] * filters[j][25];
            spectrogram[i][j] += power_spectrum[i][26] * filters[j][26];
            spectrogram[i][j] += power_spectrum[i][27] * filters[j][27];
            spectrogram[i][j] += power_spectrum[i][28] * filters[j][28];
            spectrogram[i][j] += power_spectrum[i][29] * filters[j][29];
            spectrogram[i][j] += power_spectrum[i][30] * filters[j][30];
            spectrogram[i][j] += power_spectrum[i][31] * filters[j][31];
            spectrogram[i][j] += power_spectrum[i][32] * filters[j][32];
            spectrogram[i][j] += power_spectrum[i][33] * filters[j][33];
            spectrogram[i][j] += power_spectrum[i][34] * filters[j][34];
            spectrogram[i][j] += power_spectrum[i][35] * filters[j][35];
            spectrogram[i][j] += power_spectrum[i][36] * filters[j][36];
            spectrogram[i][j] += power_spectrum[i][37] * filters[j][37];
            spectrogram[i][j] += power_spectrum[i][38] * filters[j][38];
            spectrogram[i][j] += power_spectrum[i][39] * filters[j][39];
            spectrogram[i][j] += power_spectrum[i][40] * filters[j][40];
            spectrogram[i][j] += power_spectrum[i][41] * filters[j][41];
            spectrogram[i][j] += power_spectrum[i][42] * filters[j][42];
            spectrogram[i][j] += power_spectrum[i][43] * filters[j][43];
            spectrogram[i][j] += power_spectrum[i][44] * filters[j][44];
            spectrogram[i][j] += power_spectrum[i][45] * filters[j][45];
            spectrogram[i][j] += power_spectrum[i][46] * filters[j][46];
            spectrogram[i][j] += power_spectrum[i][47] * filters[j][47];
            spectrogram[i][j] += power_spectrum[i][48] * filters[j][48];
            spectrogram[i][j] += power_spectrum[i][49] * filters[j][49];
            spectrogram[i][j] += power_spectrum[i][50] * filters[j][50];
            spectrogram[i][j] += power_spectrum[i][51] * filters[j][51];
            spectrogram[i][j] += power_spectrum[i][52] * filters[j][52];
            spectrogram[i][j] += power_spectrum[i][53] * filters[j][53];
            spectrogram[i][j] += power_spectrum[i][54] * filters[j][54];
            spectrogram[i][j] += power_spectrum[i][55] * filters[j][55];
            spectrogram[i][j] += power_spectrum[i][56] * filters[j][56];
            spectrogram[i][j] += power_spectrum[i][57] * filters[j][57];
            spectrogram[i][j] += power_spectrum[i][58] * filters[j][58];
            spectrogram[i][j] += power_spectrum[i][59] * filters[j][59];
            spectrogram[i][j] += power_spectrum[i][60] * filters[j][60];
            spectrogram[i][j] += power_spectrum[i][61] * filters[j][61];
            spectrogram[i][j] += power_spectrum[i][62] * filters[j][62];
            spectrogram[i][j] += power_spectrum[i][63] * filters[j][63];
            spectrogram[i][j] += power_spectrum[i][64] * filters[j][64];
            spectrogram[i][j] += power_spectrum[i][65] * filters[j][65];
            spectrogram[i][j] += power_spectrum[i][66] * filters[j][66];
            spectrogram[i][j] += power_spectrum[i][67] * filters[j][67];
            spectrogram[i][j] += power_spectrum[i][68] * filters[j][68];
            spectrogram[i][j] += power_spectrum[i][69] * filters[j][69];
            spectrogram[i][j] += power_spectrum[i][70] * filters[j][70];
            spectrogram[i][j] += power_spectrum[i][71] * filters[j][71];
            spectrogram[i][j] += power_spectrum[i][72] * filters[j][72];
            spectrogram[i][j] += power_spectrum[i][73] * filters[j][73];
            spectrogram[i][j] += power_spectrum[i][74] * filters[j][74];
            spectrogram[i][j] += power_spectrum[i][75] * filters[j][75];
            spectrogram[i][j] += power_spectrum[i][76] * filters[j][76];
            spectrogram[i][j] += power_spectrum[i][77] * filters[j][77];
            spectrogram[i][j] += power_spectrum[i][78] * filters[j][78];
            spectrogram[i][j] += power_spectrum[i][79] * filters[j][79];
            spectrogram[i][j] += power_spectrum[i][80] * filters[j][80];
            spectrogram[i][j] += power_spectrum[i][81] * filters[j][81];
            spectrogram[i][j] += power_spectrum[i][82] * filters[j][82];
            spectrogram[i][j] += power_spectrum[i][83] * filters[j][83];
            spectrogram[i][j] += power_spectrum[i][84] * filters[j][84];
            spectrogram[i][j] += power_spectrum[i][85] * filters[j][85];
            spectrogram[i][j] += power_spectrum[i][86] * filters[j][86];
            spectrogram[i][j] += power_spectrum[i][87] * filters[j][87];
            spectrogram[i][j] += power_spectrum[i][88] * filters[j][88];
            spectrogram[i][j] += power_spectrum[i][89] * filters[j][89];
            spectrogram[i][j] += power_spectrum[i][90] * filters[j][90];
            spectrogram[i][j] += power_spectrum[i][91] * filters[j][91];
            spectrogram[i][j] += power_spectrum[i][92] * filters[j][92];
            spectrogram[i][j] += power_spectrum[i][93] * filters[j][93];
            spectrogram[i][j] += power_spectrum[i][94] * filters[j][94];
            spectrogram[i][j] += power_spectrum[i][95] * filters[j][95];
            spectrogram[i][j] += power_spectrum[i][96] * filters[j][96];
            spectrogram[i][j] += power_spectrum[i][97] * filters[j][97];
            spectrogram[i][j] += power_spectrum[i][98] * filters[j][98];
            spectrogram[i][j] += power_spectrum[i][99] * filters[j][99];
            spectrogram[i][j] += power_spectrum[i][100] * filters[j][100];
            spectrogram[i][j] += power_spectrum[i][101] * filters[j][101];
            spectrogram[i][j] += power_spectrum[i][102] * filters[j][102];
            spectrogram[i][j] += power_spectrum[i][103] * filters[j][103];
            spectrogram[i][j] += power_spectrum[i][104] * filters[j][104];
            spectrogram[i][j] += power_spectrum[i][105] * filters[j][105];
            spectrogram[i][j] += power_spectrum[i][106] * filters[j][106];
            spectrogram[i][j] += power_spectrum[i][107] * filters[j][107];
            spectrogram[i][j] += power_spectrum[i][108] * filters[j][108];
            spectrogram[i][j] += power_spectrum[i][109] * filters[j][109];
            spectrogram[i][j] += power_spectrum[i][110] * filters[j][110];
            spectrogram[i][j] += power_spectrum[i][111] * filters[j][111];
            spectrogram[i][j] += power_spectrum[i][112] * filters[j][112];
            spectrogram[i][j] += power_spectrum[i][113] * filters[j][113];
            spectrogram[i][j] += power_spectrum[i][114] * filters[j][114];
            spectrogram[i][j] += power_spectrum[i][115] * filters[j][115];
            spectrogram[i][j] += power_spectrum[i][116] * filters[j][116];
            spectrogram[i][j] += power_spectrum[i][117] * filters[j][117];
            spectrogram[i][j] += power_spectrum[i][118] * filters[j][118];
            spectrogram[i][j] += power_spectrum[i][119] * filters[j][119];
            spectrogram[i][j] += power_spectrum[i][120] * filters[j][120];
            spectrogram[i][j] += power_spectrum[i][121] * filters[j][121];
            spectrogram[i][j] += power_spectrum[i][122] * filters[j][122];
            spectrogram[i][j] += power_spectrum[i][123] * filters[j][123];
            spectrogram[i][j] += power_spectrum[i][124] * filters[j][124];
            spectrogram[i][j] += power_spectrum[i][125] * filters[j][125];
            spectrogram[i][j] += power_spectrum[i][126] * filters[j][126];
            spectrogram[i][j] += power_spectrum[i][127] * filters[j][127];
            spectrogram[i][j] += power_spectrum[i][128] * filters[j][128];
            spectrogram[i][j] += power_spectrum[i][129] * filters[j][129];
            spectrogram[i][j] += power_spectrum[i][130] * filters[j][130];
            spectrogram[i][j] += power_spectrum[i][131] * filters[j][131];
            spectrogram[i][j] += power_spectrum[i][132] * filters[j][132];
            spectrogram[i][j] += power_spectrum[i][133] * filters[j][133];
            spectrogram[i][j] += power_spectrum[i][134] * filters[j][134];
            spectrogram[i][j] += power_spectrum[i][135] * filters[j][135];
            spectrogram[i][j] += power_spectrum[i][136] * filters[j][136];
            spectrogram[i][j] += power_spectrum[i][137] * filters[j][137];
            spectrogram[i][j] += power_spectrum[i][138] * filters[j][138];
            spectrogram[i][j] += power_spectrum[i][139] * filters[j][139];
            spectrogram[i][j] += power_spectrum[i][140] * filters[j][140];
            spectrogram[i][j] += power_spectrum[i][141] * filters[j][141];
            spectrogram[i][j] += power_spectrum[i][142] * filters[j][142];
            spectrogram[i][j] += power_spectrum[i][143] * filters[j][143];
            spectrogram[i][j] += power_spectrum[i][144] * filters[j][144];
            spectrogram[i][j] += power_spectrum[i][145] * filters[j][145];
            spectrogram[i][j] += power_spectrum[i][146] * filters[j][146];
            spectrogram[i][j] += power_spectrum[i][147] * filters[j][147];
            spectrogram[i][j] += power_spectrum[i][148] * filters[j][148];
            spectrogram[i][j] += power_spectrum[i][149] * filters[j][149];
            spectrogram[i][j] += power_spectrum[i][150] * filters[j][150];
            spectrogram[i][j] += power_spectrum[i][151] * filters[j][151];
            spectrogram[i][j] += power_spectrum[i][152] * filters[j][152];
            spectrogram[i][j] += power_spectrum[i][153] * filters[j][153];
            spectrogram[i][j] += power_spectrum[i][154] * filters[j][154];
            spectrogram[i][j] += power_spectrum[i][155] * filters[j][155];
            spectrogram[i][j] += power_spectrum[i][156] * filters[j][156];
            spectrogram[i][j] += power_spectrum[i][157] * filters[j][157];
            spectrogram[i][j] += power_spectrum[i][158] * filters[j][158];
            spectrogram[i][j] += power_spectrum[i][159] * filters[j][159];
            spectrogram[i][j] += power_spectrum[i][160] * filters[j][160];
            spectrogram[i][j] += power_spectrum[i][161] * filters[j][161];
            spectrogram[i][j] += power_spectrum[i][162] * filters[j][162];
            spectrogram[i][j] += power_spectrum[i][163] * filters[j][163];
            spectrogram[i][j] += power_spectrum[i][164] * filters[j][164];
            spectrogram[i][j] += power_spectrum[i][165] * filters[j][165];
            spectrogram[i][j] += power_spectrum[i][166] * filters[j][166];
            spectrogram[i][j] += power_spectrum[i][167] * filters[j][167];
            spectrogram[i][j] += power_spectrum[i][168] * filters[j][168];
            spectrogram[i][j] += power_spectrum[i][169] * filters[j][169];
            spectrogram[i][j] += power_spectrum[i][170] * filters[j][170];
            spectrogram[i][j] += power_spectrum[i][171] * filters[j][171];
            spectrogram[i][j] += power_spectrum[i][172] * filters[j][172];
            spectrogram[i][j] += power_spectrum[i][173] * filters[j][173];
            spectrogram[i][j] += power_spectrum[i][174] * filters[j][174];
            spectrogram[i][j] += power_spectrum[i][175] * filters[j][175];
            spectrogram[i][j] += power_spectrum[i][176] * filters[j][176];
            spectrogram[i][j] += power_spectrum[i][177] * filters[j][177];
            spectrogram[i][j] += power_spectrum[i][178] * filters[j][178];
            spectrogram[i][j] += power_spectrum[i][179] * filters[j][179];
            spectrogram[i][j] += power_spectrum[i][180] * filters[j][180];
            spectrogram[i][j] += power_spectrum[i][181] * filters[j][181];
            spectrogram[i][j] += power_spectrum[i][182] * filters[j][182];
            spectrogram[i][j] += power_spectrum[i][183] * filters[j][183];
            spectrogram[i][j] += power_spectrum[i][184] * filters[j][184];
            spectrogram[i][j] += power_spectrum[i][185] * filters[j][185];
            spectrogram[i][j] += power_spectrum[i][186] * filters[j][186];
            spectrogram[i][j] += power_spectrum[i][187] * filters[j][187];
            spectrogram[i][j] += power_spectrum[i][188] * filters[j][188];
            spectrogram[i][j] += power_spectrum[i][189] * filters[j][189];
            spectrogram[i][j] += power_spectrum[i][190] * filters[j][190];
            spectrogram[i][j] += power_spectrum[i][191] * filters[j][191];
            spectrogram[i][j] += power_spectrum[i][192] * filters[j][192];
            spectrogram[i][j] += power_spectrum[i][193] * filters[j][193];
            spectrogram[i][j] += power_spectrum[i][194] * filters[j][194];
            spectrogram[i][j] += power_spectrum[i][195] * filters[j][195];

            spectrogram[i][j] += power_spectrum[i][196] * filters[j][196];
            spectrogram[i][j] += power_spectrum[i][197] * filters[j][197];
            spectrogram[i][j] += power_spectrum[i][198] * filters[j][198];
            spectrogram[i][j] += power_spectrum[i][199] * filters[j][199];

            spectrogram[i][j] += power_spectrum[i][200] * filters[j][200];
            spectrogram[i][j] += 0.0 * 0.0;
            spectrogram[i][j] += 0.0 * 0.0;
            spectrogram[i][j] += 0.0 * 0.0;
        }
    }
    return spectrogram;
}

#[cfg(target_arch = "aarch64")]
/// I'm a monster
pub(crate) fn compute_mel(
    power_spectrum: &[Vec<f32>],
    filters: &[Vec<f32>],
    n_mel_frames: usize,
    mel_bins: usize,
) -> Vec<Vec<f32>> {
    use std::arch::aarch64::{vdupq_n_f32, vfmaq_f32, vgetq_lane_f32, vld1q_f32_x4};

    let mut pwr = [0.0; 208];
    let mut flt = [0.0; 208];

    let mut spectrogram = vec![vec![0.0; mel_bins]; n_mel_frames];
    for i in 0..n_mel_frames {
        unsafe {
            pwr[0..N_FFT].copy_from_slice(&power_spectrum[i]);
            let pwr_1 = vld1q_f32_x4(pwr[0..16].as_ptr());
            let pwr_2 = vld1q_f32_x4(pwr[16..32].as_ptr());
            let pwr_3 = vld1q_f32_x4(pwr[32..48].as_ptr());
            let pwr_4 = vld1q_f32_x4(pwr[48..64].as_ptr());
            let pwr_5 = vld1q_f32_x4(pwr[64..80].as_ptr());
            let pwr_6 = vld1q_f32_x4(pwr[80..96].as_ptr());
            let pwr_7 = vld1q_f32_x4(pwr[96..112].as_ptr());
            let pwr_8 = vld1q_f32_x4(pwr[112..128].as_ptr());
            let pwr_9 = vld1q_f32_x4(pwr[128..144].as_ptr());
            let pwr_10 = vld1q_f32_x4(pwr[144..160].as_ptr());
            let pwr_11 = vld1q_f32_x4(pwr[160..176].as_ptr());
            let pwr_12 = vld1q_f32_x4(pwr[176..192].as_ptr());
            let pwr_13 = vld1q_f32_x4(pwr[192..208].as_ptr());
            for j in 0..mel_bins {
                flt[0..N_FFT].copy_from_slice(&filters[j]);

                let flt_16_1 = vld1q_f32_x4(flt[0..16].as_ptr());
                let flt_16_2 = vld1q_f32_x4(flt[16..32].as_ptr());
                let flt_16_3 = vld1q_f32_x4(flt[32..48].as_ptr());
                let flt_16_4 = vld1q_f32_x4(flt[48..64].as_ptr());
                let flt_16_5 = vld1q_f32_x4(flt[64..80].as_ptr());
                let flt_16_6 = vld1q_f32_x4(flt[80..96].as_ptr());
                let flt_16_7 = vld1q_f32_x4(flt[96..112].as_ptr());
                let flt_16_8 = vld1q_f32_x4(flt[112..128].as_ptr());
                let flt_16_9 = vld1q_f32_x4(flt[128..144].as_ptr());
                let flt_16_10 = vld1q_f32_x4(flt[144..160].as_ptr());
                let flt_16_11 = vld1q_f32_x4(flt[160..176].as_ptr());
                let flt_16_12 = vld1q_f32_x4(flt[176..192].as_ptr());
                let flt_16_13 = vld1q_f32_x4(flt[192..208].as_ptr());

                let res_16_1_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_1.0, flt_16_1.0);
                let res_16_1_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_1.1, flt_16_1.1);
                let res_16_1_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_1.2, flt_16_1.2);
                let res_16_1_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_1.3, flt_16_1.3);
                let res_16_2_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_2.0, flt_16_2.0);
                let res_16_2_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_2.1, flt_16_2.1);
                let res_16_2_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_2.2, flt_16_2.2);
                let res_16_2_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_2.3, flt_16_2.3);
                let res_16_3_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_3.0, flt_16_3.0);
                let res_16_3_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_3.1, flt_16_3.1);
                let res_16_3_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_3.2, flt_16_3.2);
                let res_16_3_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_3.3, flt_16_3.3);
                let res_16_4_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_4.0, flt_16_4.0);
                let res_16_4_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_4.1, flt_16_4.1);
                let res_16_4_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_4.2, flt_16_4.2);
                let res_16_4_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_4.3, flt_16_4.3);
                let res_16_5_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_5.0, flt_16_5.0);
                let res_16_5_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_5.1, flt_16_5.1);
                let res_16_5_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_5.2, flt_16_5.2);
                let res_16_5_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_5.3, flt_16_5.3);
                let res_16_6_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_6.0, flt_16_6.0);
                let res_16_6_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_6.1, flt_16_6.1);
                let res_16_6_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_6.2, flt_16_6.2);
                let res_16_6_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_6.3, flt_16_6.3);
                let res_16_7_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_7.0, flt_16_7.0);
                let res_16_7_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_7.1, flt_16_7.1);
                let res_16_7_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_7.2, flt_16_7.2);
                let res_16_7_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_7.3, flt_16_7.3);
                let res_16_8_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_8.0, flt_16_8.0);
                let res_16_8_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_8.1, flt_16_8.1);
                let res_16_8_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_8.2, flt_16_8.2);
                let res_16_8_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_8.3, flt_16_8.3);
                let res_16_9_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_9.0, flt_16_9.0);
                let res_16_9_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_9.1, flt_16_9.1);
                let res_16_9_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_9.2, flt_16_9.2);
                let res_16_9_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_9.3, flt_16_9.3);
                let res_16_10_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_10.0, flt_16_10.0);
                let res_16_10_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_10.1, flt_16_10.1);
                let res_16_10_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_10.2, flt_16_10.2);
                let res_16_10_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_10.3, flt_16_10.3);
                let res_16_11_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_11.0, flt_16_11.0);
                let res_16_11_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_11.1, flt_16_11.1);
                let res_16_11_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_11.2, flt_16_11.2);
                let res_16_11_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_11.3, flt_16_11.3);
                let res_16_12_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_12.0, flt_16_12.0);
                let res_16_12_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_12.1, flt_16_12.1);
                let res_16_12_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_12.2, flt_16_12.2);
                let res_16_12_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_12.3, flt_16_12.3);
                let res_16_13_1 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_13.0, flt_16_13.0);
                let res_16_13_2 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_13.1, flt_16_13.1);
                let res_16_13_3 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_13.2, flt_16_13.2);
                let res_16_13_4 = vfmaq_f32(vdupq_n_f32(0.0f32), pwr_13.3, flt_16_13.3);
                spectrogram[i][j] = vgetq_lane_f32(res_16_1_1, 0)
                    + vgetq_lane_f32(res_16_1_1, 1)
                    + vgetq_lane_f32(res_16_1_1, 2)
                    + vgetq_lane_f32(res_16_1_1, 3)
                    + vgetq_lane_f32(res_16_1_2, 0)
                    + vgetq_lane_f32(res_16_1_2, 1)
                    + vgetq_lane_f32(res_16_1_2, 2)
                    + vgetq_lane_f32(res_16_1_2, 3)
                    + vgetq_lane_f32(res_16_1_3, 0)
                    + vgetq_lane_f32(res_16_1_3, 1)
                    + vgetq_lane_f32(res_16_1_3, 2)
                    + vgetq_lane_f32(res_16_1_3, 3)
                    + vgetq_lane_f32(res_16_1_4, 0)
                    + vgetq_lane_f32(res_16_1_4, 1)
                    + vgetq_lane_f32(res_16_1_4, 2)
                    + vgetq_lane_f32(res_16_1_4, 3)
                    + vgetq_lane_f32(res_16_2_1, 0)
                    + vgetq_lane_f32(res_16_2_1, 1)
                    + vgetq_lane_f32(res_16_2_1, 2)
                    + vgetq_lane_f32(res_16_2_1, 3)
                    + vgetq_lane_f32(res_16_2_2, 0)
                    + vgetq_lane_f32(res_16_2_2, 1)
                    + vgetq_lane_f32(res_16_2_2, 2)
                    + vgetq_lane_f32(res_16_2_2, 3)
                    + vgetq_lane_f32(res_16_2_3, 0)
                    + vgetq_lane_f32(res_16_2_3, 1)
                    + vgetq_lane_f32(res_16_2_3, 2)
                    + vgetq_lane_f32(res_16_2_3, 3)
                    + vgetq_lane_f32(res_16_2_4, 0)
                    + vgetq_lane_f32(res_16_2_4, 1)
                    + vgetq_lane_f32(res_16_2_4, 2)
                    + vgetq_lane_f32(res_16_2_4, 3)
                    + vgetq_lane_f32(res_16_3_1, 0)
                    + vgetq_lane_f32(res_16_3_1, 1)
                    + vgetq_lane_f32(res_16_3_1, 2)
                    + vgetq_lane_f32(res_16_3_1, 3)
                    + vgetq_lane_f32(res_16_3_2, 0)
                    + vgetq_lane_f32(res_16_3_2, 1)
                    + vgetq_lane_f32(res_16_3_2, 2)
                    + vgetq_lane_f32(res_16_3_2, 3)
                    + vgetq_lane_f32(res_16_3_3, 0)
                    + vgetq_lane_f32(res_16_3_3, 1)
                    + vgetq_lane_f32(res_16_3_3, 2)
                    + vgetq_lane_f32(res_16_3_3, 3)
                    + vgetq_lane_f32(res_16_3_4, 0)
                    + vgetq_lane_f32(res_16_3_4, 1)
                    + vgetq_lane_f32(res_16_3_4, 2)
                    + vgetq_lane_f32(res_16_3_4, 3)
                    + vgetq_lane_f32(res_16_4_1, 0)
                    + vgetq_lane_f32(res_16_4_1, 1)
                    + vgetq_lane_f32(res_16_4_1, 2)
                    + vgetq_lane_f32(res_16_4_1, 3)
                    + vgetq_lane_f32(res_16_4_2, 0)
                    + vgetq_lane_f32(res_16_4_2, 1)
                    + vgetq_lane_f32(res_16_4_2, 2)
                    + vgetq_lane_f32(res_16_4_2, 3)
                    + vgetq_lane_f32(res_16_4_3, 0)
                    + vgetq_lane_f32(res_16_4_3, 1)
                    + vgetq_lane_f32(res_16_4_3, 2)
                    + vgetq_lane_f32(res_16_4_3, 3)
                    + vgetq_lane_f32(res_16_4_4, 0)
                    + vgetq_lane_f32(res_16_4_4, 1)
                    + vgetq_lane_f32(res_16_4_4, 2)
                    + vgetq_lane_f32(res_16_4_4, 3)
                    + vgetq_lane_f32(res_16_5_1, 0)
                    + vgetq_lane_f32(res_16_5_1, 1)
                    + vgetq_lane_f32(res_16_5_1, 2)
                    + vgetq_lane_f32(res_16_5_1, 3)
                    + vgetq_lane_f32(res_16_5_2, 0)
                    + vgetq_lane_f32(res_16_5_2, 1)
                    + vgetq_lane_f32(res_16_5_2, 2)
                    + vgetq_lane_f32(res_16_5_2, 3)
                    + vgetq_lane_f32(res_16_5_3, 0)
                    + vgetq_lane_f32(res_16_5_3, 1)
                    + vgetq_lane_f32(res_16_5_3, 2)
                    + vgetq_lane_f32(res_16_5_3, 3)
                    + vgetq_lane_f32(res_16_5_4, 0)
                    + vgetq_lane_f32(res_16_5_4, 1)
                    + vgetq_lane_f32(res_16_5_4, 2)
                    + vgetq_lane_f32(res_16_5_4, 3)
                    + vgetq_lane_f32(res_16_6_1, 0)
                    + vgetq_lane_f32(res_16_6_1, 1)
                    + vgetq_lane_f32(res_16_6_1, 2)
                    + vgetq_lane_f32(res_16_6_1, 3)
                    + vgetq_lane_f32(res_16_6_2, 0)
                    + vgetq_lane_f32(res_16_6_2, 1)
                    + vgetq_lane_f32(res_16_6_2, 2)
                    + vgetq_lane_f32(res_16_6_2, 3)
                    + vgetq_lane_f32(res_16_6_3, 0)
                    + vgetq_lane_f32(res_16_6_3, 1)
                    + vgetq_lane_f32(res_16_6_3, 2)
                    + vgetq_lane_f32(res_16_6_3, 3)
                    + vgetq_lane_f32(res_16_6_4, 0)
                    + vgetq_lane_f32(res_16_6_4, 1)
                    + vgetq_lane_f32(res_16_6_4, 2)
                    + vgetq_lane_f32(res_16_6_4, 3)
                    + vgetq_lane_f32(res_16_7_1, 0)
                    + vgetq_lane_f32(res_16_7_1, 1)
                    + vgetq_lane_f32(res_16_7_1, 2)
                    + vgetq_lane_f32(res_16_7_1, 3)
                    + vgetq_lane_f32(res_16_7_2, 0)
                    + vgetq_lane_f32(res_16_7_2, 1)
                    + vgetq_lane_f32(res_16_7_2, 2)
                    + vgetq_lane_f32(res_16_7_2, 3)
                    + vgetq_lane_f32(res_16_7_3, 0)
                    + vgetq_lane_f32(res_16_7_3, 1)
                    + vgetq_lane_f32(res_16_7_3, 2)
                    + vgetq_lane_f32(res_16_7_3, 3)
                    + vgetq_lane_f32(res_16_7_4, 0)
                    + vgetq_lane_f32(res_16_7_4, 1)
                    + vgetq_lane_f32(res_16_7_4, 2)
                    + vgetq_lane_f32(res_16_7_4, 3)
                    + vgetq_lane_f32(res_16_8_1, 0)
                    + vgetq_lane_f32(res_16_8_1, 1)
                    + vgetq_lane_f32(res_16_8_1, 2)
                    + vgetq_lane_f32(res_16_8_1, 3)
                    + vgetq_lane_f32(res_16_8_2, 0)
                    + vgetq_lane_f32(res_16_8_2, 1)
                    + vgetq_lane_f32(res_16_8_2, 2)
                    + vgetq_lane_f32(res_16_8_2, 3)
                    + vgetq_lane_f32(res_16_8_3, 0)
                    + vgetq_lane_f32(res_16_8_3, 1)
                    + vgetq_lane_f32(res_16_8_3, 2)
                    + vgetq_lane_f32(res_16_8_3, 3)
                    + vgetq_lane_f32(res_16_8_4, 0)
                    + vgetq_lane_f32(res_16_8_4, 1)
                    + vgetq_lane_f32(res_16_8_4, 2)
                    + vgetq_lane_f32(res_16_8_4, 3)
                    + vgetq_lane_f32(res_16_9_1, 0)
                    + vgetq_lane_f32(res_16_9_1, 1)
                    + vgetq_lane_f32(res_16_9_1, 2)
                    + vgetq_lane_f32(res_16_9_1, 3)
                    + vgetq_lane_f32(res_16_9_2, 0)
                    + vgetq_lane_f32(res_16_9_2, 1)
                    + vgetq_lane_f32(res_16_9_2, 2)
                    + vgetq_lane_f32(res_16_9_2, 3)
                    + vgetq_lane_f32(res_16_9_3, 0)
                    + vgetq_lane_f32(res_16_9_3, 1)
                    + vgetq_lane_f32(res_16_9_3, 2)
                    + vgetq_lane_f32(res_16_9_3, 3)
                    + vgetq_lane_f32(res_16_9_4, 0)
                    + vgetq_lane_f32(res_16_9_4, 1)
                    + vgetq_lane_f32(res_16_9_4, 2)
                    + vgetq_lane_f32(res_16_9_4, 3)
                    + vgetq_lane_f32(res_16_10_1, 0)
                    + vgetq_lane_f32(res_16_10_1, 1)
                    + vgetq_lane_f32(res_16_10_1, 2)
                    + vgetq_lane_f32(res_16_10_1, 3)
                    + vgetq_lane_f32(res_16_10_2, 0)
                    + vgetq_lane_f32(res_16_10_2, 1)
                    + vgetq_lane_f32(res_16_10_2, 2)
                    + vgetq_lane_f32(res_16_10_2, 3)
                    + vgetq_lane_f32(res_16_10_3, 0)
                    + vgetq_lane_f32(res_16_10_3, 1)
                    + vgetq_lane_f32(res_16_10_3, 2)
                    + vgetq_lane_f32(res_16_10_3, 3)
                    + vgetq_lane_f32(res_16_10_4, 0)
                    + vgetq_lane_f32(res_16_10_4, 1)
                    + vgetq_lane_f32(res_16_10_4, 2)
                    + vgetq_lane_f32(res_16_10_4, 3)
                    + vgetq_lane_f32(res_16_11_1, 0)
                    + vgetq_lane_f32(res_16_11_1, 1)
                    + vgetq_lane_f32(res_16_11_1, 2)
                    + vgetq_lane_f32(res_16_11_1, 3)
                    + vgetq_lane_f32(res_16_11_2, 0)
                    + vgetq_lane_f32(res_16_11_2, 1)
                    + vgetq_lane_f32(res_16_11_2, 2)
                    + vgetq_lane_f32(res_16_11_2, 3)
                    + vgetq_lane_f32(res_16_11_3, 0)
                    + vgetq_lane_f32(res_16_11_3, 1)
                    + vgetq_lane_f32(res_16_11_3, 2)
                    + vgetq_lane_f32(res_16_11_3, 3)
                    + vgetq_lane_f32(res_16_11_4, 0)
                    + vgetq_lane_f32(res_16_11_4, 1)
                    + vgetq_lane_f32(res_16_11_4, 2)
                    + vgetq_lane_f32(res_16_11_4, 3)
                    + vgetq_lane_f32(res_16_12_1, 0)
                    + vgetq_lane_f32(res_16_12_1, 1)
                    + vgetq_lane_f32(res_16_12_1, 2)
                    + vgetq_lane_f32(res_16_12_1, 3)
                    + vgetq_lane_f32(res_16_12_2, 0)
                    + vgetq_lane_f32(res_16_12_2, 1)
                    + vgetq_lane_f32(res_16_12_2, 2)
                    + vgetq_lane_f32(res_16_12_2, 3)
                    + vgetq_lane_f32(res_16_12_3, 0)
                    + vgetq_lane_f32(res_16_12_3, 1)
                    + vgetq_lane_f32(res_16_12_3, 2)
                    + vgetq_lane_f32(res_16_12_3, 3)
                    + vgetq_lane_f32(res_16_12_4, 0)
                    + vgetq_lane_f32(res_16_12_4, 1)
                    + vgetq_lane_f32(res_16_12_4, 2)
                    + vgetq_lane_f32(res_16_12_4, 3)
                    + vgetq_lane_f32(res_16_13_1, 0)
                    + vgetq_lane_f32(res_16_13_1, 1)
                    + vgetq_lane_f32(res_16_13_1, 2)
                    + vgetq_lane_f32(res_16_13_1, 3)
                    + vgetq_lane_f32(res_16_13_2, 0)
                    + vgetq_lane_f32(res_16_13_2, 1)
                    + vgetq_lane_f32(res_16_13_2, 2)
                    + vgetq_lane_f32(res_16_13_2, 3)
                    + vgetq_lane_f32(res_16_13_3, 0)
                    + vgetq_lane_f32(res_16_13_3, 1)
                    + vgetq_lane_f32(res_16_13_3, 2)
                    + vgetq_lane_f32(res_16_13_3, 3)
                    + vgetq_lane_f32(res_16_13_4, 0)
                    + vgetq_lane_f32(res_16_13_4, 1)
                    + vgetq_lane_f32(res_16_13_4, 2)
                    + vgetq_lane_f32(res_16_13_4, 3);
            }
        }
    }
    return spectrogram;
}
