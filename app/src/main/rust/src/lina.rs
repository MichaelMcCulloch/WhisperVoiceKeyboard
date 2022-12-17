use rustfft::num_complex::Complex32;

// #[cfg(not(target_arch = "aarch64"))]
/// Calculates the dot product of two slices of `f32` values.
///
/// # Parameters
///
/// * `left` - The left slice of `f32` values.
/// * `right` - The right slice of `f32` values.
///
/// # Returns
///
/// The dot product of the two slices.
///
/// # Examples
///
/// ```
/// let left = [1.0f32, 2.0f32, 3.0f32];
/// let right = [4.0f32, 5.0f32, 6.0f32];
///
/// let result = dot_product(&left, &right);
/// assert_eq!(result, 32.0f32);
/// ```
pub(crate) fn dot_product(left: &[f32], right: &[f32]) -> f32 {
    let mut sum = 0.0f32;

    for (l, r) in left.iter().zip(right.iter()) {
        sum += *l * *r;
    }

    sum
}
#[cfg(target_arch = "aarch64")]
// #[cfg(target_arch = "aarch64")]
/// I tried to beat the compiler. I lost. Badly. I suspect the compiler detects both the loop above and the looped function calls correctly and does a proper matrix multiplication
/// Calculates the dot product of two slices of `f32` values.
///
/// # Parameters
///
/// * `left` - The left slice of `f32` values.
/// * `right` - The right slice of `f32` values.
///
/// # Returns
///
/// The dot product of the two slices.
///
/// # Examples
///
/// ```
/// let left = [1.0f32, 2.0f32, 3.0f32];
/// let right = [4.0f32, 5.0f32, 6.0f32];
///
/// let result = dot_product(&left, &right);
/// assert_eq!(result, 32.0f32);
/// ```
pub(crate) fn dot_productSLOW(left: &[f32], right: &[f32]) -> f32 {
    // log::info!("{}x{}", left.len(), right.len());
    use std::arch::aarch64::{
        vdupq_n_f32, vfmaq_f32, vgetq_lane_f32, vld1_f32_x4, vld1q_f32, vld1q_f32_x4,
    };
    // We'll pad the arrays so that their lengths are divisible by 4
    let pad_length_l = 4 - (left.len() % 4);
    let pad_length_r = 4 - (right.len() % 4);
    // Create the padded arrays
    let mut pad_left = vec![0.0f32; left.len() + pad_length_l];
    let mut pad_right = vec![0.0f32; right.len() + pad_length_r];
    // Copy the contents of the original arrays into the padded arrays.
    pad_left[0..left.len()].copy_from_slice(left);
    pad_right[0..right.len()].copy_from_slice(right);
    // Initialize accumulator to 0.0
    let zero = unsafe { vdupq_n_f32(0.0f32) };
    // Perform the dot product using 4-element SIMD instructions
    let result = pad_left
        .chunks_exact(4)
        .zip(pad_right.chunks_exact(4))
        .into_iter()
        .fold(zero, |acc, (left, right)| unsafe {
            // Load the 4 elements for each array into a single SIMD vector
            let l = vld1q_f32(left.as_ptr());
            let r = vld1q_f32(right.as_ptr());
            // Perform 4-element multiply-accumulate
            vfmaq_f32(acc, l, r)
        });

    // Sum the 4 elements in the result vector
    let result = unsafe {
        vgetq_lane_f32(result, 0)
            + vgetq_lane_f32(result, 1)
            + vgetq_lane_f32(result, 2)
            + vgetq_lane_f32(result, 3)
    };
    result
}

fn dft(input: &Vec<f32>) -> Vec<Complex32> {
    let length = input.len();

    let mut output = Vec::with_capacity(length);

    for k in 0..length {
        let mut re: f32 = 0.0;
        let mut im: f32 = 0.0;
        for n in 0..length {
            let angle = 2.0 * std::f32::consts::PI * (k as f32) * (n as f32) / (length as f32);
            re += input[n] * angle.cos();
            im -= input[n] * angle.sin();
        }

        output.push(Complex32::new(re, im));
    }

    output
}
pub(crate) fn fft(input: &Vec<f32>) -> Vec<Complex32> {
    log::info!("input.len {}", input.len());
    let length = input.len();

    let mut output = vec![Complex32::new(0.0, 0.0); length];

    if length == 1 {
        return vec![Complex32::new(input[0], 0.0)];
    }

    if length % 2 == 1 {
        let dft = dft(input);
        log::info!("dft");
        return dft;
    }

    let mut even = Vec::with_capacity(length / 2);
    let mut odd = Vec::with_capacity(length / 2);

    for (i, elem) in input.iter().enumerate() {
        if i % 2 == 0 {
            even.push(*elem);
        } else {
            odd.push(*elem);
        }
    }

    let even_fft = fft(&even);
    let odd_fft = fft(&odd);

    for k in 0..(length / 2) {
        let theta = 2.0 * std::f32::consts::PI * (k as f32) / (length as f32);

        let re = theta.cos();
        let im = -theta.sin();

        let re_odd = odd_fft[k].re;
        let im_odd = odd_fft[k].im;

        output[k] = Complex32::new(
            even_fft[k].re + re * re_odd - im * im_odd,
            even_fft[k].im + re * im_odd + im * re_odd,
        );
        output[(k + length / 2)] = Complex32::new(
            even_fft[k].re - re * re_odd + im * im_odd,
            even_fft[k].im - re * im_odd - im * re_odd,
        );
    }
    output
}
