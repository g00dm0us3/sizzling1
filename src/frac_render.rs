use std::ops::RangeInclusive;

use image::{Rgba, RgbaImage};

use crate::{ds::array_2d::{Array2D, Index2D}, util};

pub(crate) struct RgbRenderer;

impl RgbRenderer {
    const LUM_LEVELS: usize = 10_000;

    pub(crate) fn img_bw_simple(array: &Array2D) -> RgbaImage {
        let mut img = RgbaImage::new(array.width() as u32, array.height() as u32);

        let width = array.width();
        let height = array.height();
        let log_max = Self::find_log_max(array);

        let avg: f32 = 0.0;
        let avg_sq: f32 = 0.0;
        let mut cnt: f32 = 0.0;

        //let cdf = Self::smoothed_cdf(array, log_max, &alpha_range);
        for x in 0..width {
            for y in 0..height {
                let density_val = array[Index2D::from(x, y)];

                let color = if density_val > f32::EPSILON {
                    let alpha = Self::density_to_alpha(density_val, log_max);

                    cnt += 1.0;
                    Rgba::from([
                        (27.0 + 27.0 * alpha).round() as u8,
                        (127.0 + 127.0 * alpha).round() as u8,
                        0,
                        255
                    ])
                } else {
                    Rgba::from([0; 4])
                };

                img.put_pixel(
                    (width - 1 - x) as u32,
                    (height - 1 - y) as u32,
                    color
                );
            }
        }

        let var = (avg_sq/cnt) -  (avg/cnt).powf(2.0);
        println!("Avg: {}, Var: {}, STD: {}",  avg / cnt, var, var.abs().sqrt());

        return img;
    }

    // - TODO: refactor.
    pub(crate) fn img_bw_(array: &Array2D) -> RgbaImage {
        let mut img = RgbaImage::new(array.width() as u32, array.height() as u32);

        let width = array.width();
        let height = array.height();
        let log_max = Self::find_log_max(array);

        let mut avg = 0.0;
        let mut avg_sq = 0.0;
        let mut cnt = 0.0;

        let alpha_range = Self::range(
            array,
            true,
            |val| Self::density_to_alpha(val, log_max)
        );
        let cdf = Self::non_smoothed_cdf(array, log_max, &alpha_range);

        //let cdf = Self::smoothed_cdf(array, log_max, &alpha_range);
        for x in 0..width {
            for y in 0..height {
                let density_val = array[Index2D::from(x, y)];

                let color = if density_val > f32::EPSILON {
                    let mut alpha = Self::density_to_alpha(density_val, log_max);

                    let cdf_bin = Self::bin_idx(alpha, &alpha_range);

                    // Histogram equalization
                    alpha = cdf[cdf_bin];

                    // Huang et. al. "Efficient Contrast Enhancement Using Adaptive Gamma Correction With Weighted Distribution"
                    //alpha = (alpha).powf(1.0 - cdf[cdf_bin]);
                    // Statistics
                    avg += alpha;
                    avg_sq += alpha * alpha;
                    cnt += 1.0;
                    //println!("{}", alpha);
                    Rgba::from([
                        (27.0 + 27.0 * alpha).round() as u8,
                        (127.0 + 127.0 * alpha).round() as u8,
                        0,
                        255
                         ])
                } else {
                    Rgba::from([0; 4])
                };

                img.put_pixel(
                    (width - 1 - x) as u32,
                    (height - 1 - y) as u32,
                     color
                );
            }
        }

        let var = (avg_sq/cnt) -  (avg/cnt).powf(2.0);
        println!("Range [{}..{}], Avg: {}, Var: {}, STD: {}", alpha_range.start(), alpha_range.end(), avg / cnt, var, var.abs().sqrt());

        return img;
    }

    fn non_smoothed_cdf(array: &Array2D, log_max: f32, alpha_range: &RangeInclusive<f32>) -> [f32; Self::LUM_LEVELS] {
        let mut pdf = Self::pdf(array, log_max, alpha_range);
        // - TODO: how to write this w/o for ?
        for i in 1..pdf.len() {
            pdf[i] += pdf[i-1];
        }

        pdf[pdf.len() - 1] = 1.0;
        pdf
    }

    fn smoothed_cdf(array: &Array2D, log_max: f32, alpha_range: &RangeInclusive<f32>) -> [f32; Self::LUM_LEVELS] {
        let pdf = Self::pdf(array, log_max, alpha_range);

        let mut pdf_max = 0.0;
        let mut pdf_min = 1.0;

        for val in pdf {
            pdf_max = val.max(pdf_max);
            pdf_min = val.min(pdf_min);
        }

        let mut pdf_weighted = [0.0; Self::LUM_LEVELS];

        const ALPHA: f32 = 0.75;

        for (idx, val) in pdf.iter().enumerate() {
            pdf_weighted[idx] = pdf_max*(((val - pdf_min).max(0.0))/(pdf_max - pdf_min)).powf(ALPHA);
        }

        let sum: f32 = pdf_weighted.iter().sum();

        pdf_weighted[0] = pdf_weighted[0] / sum;
        for l in 1..Self::LUM_LEVELS {
            pdf_weighted[l] += pdf_weighted[l - 1] / sum;
        }

        pdf_weighted
    }

    fn density_to_alpha(density: f32, log_max: f32)-> f32 {
        let logged = -1.0*(density).log10();
        logged / log_max
    }

    fn pdf(array: &Array2D, log_max: f32, alpha_range: &RangeInclusive<f32>) -> [f32; Self::LUM_LEVELS] {
        let mut pdf = [0.0; Self::LUM_LEVELS];
        let mut cnt = 1.0;

        for x in 0..array.width() {
            for y in 0..array.height() {
                let density_val = array[Index2D::from(x, y)];
                if density_val > f32::EPSILON {
                    let alpha = Self::density_to_alpha(density_val, log_max);
                    let bin_idx = Self::bin_idx(alpha, &alpha_range);
                    pdf[bin_idx] += 1.0;
                    cnt += 1.0;
                }
             }
        }

        // this is PDF
        pdf = pdf.map(|val| val / cnt);

        pdf
    }

    fn range<F>(array: &Array2D, only_non_zero: bool, transform: F) -> RangeInclusive<f32>
    where F: Fn (f32) -> f32 {
        let mut max: f32 = 0.0;
        let mut min: f32 = 1.0;

        for x in 0..array.width() {
            for y in 0..array.height() {
                let density_val = array[Index2D::from(x, y)];

                if only_non_zero && density_val < f32::EPSILON { continue; }

                //let logged = -1.0*(density_val).log10();
                //let alpha = logged / log_max;
                let alpha = transform(density_val);
                min = min.min(alpha);
                max = max.max(alpha);
             }
        }

        min..=max
    }

    fn bin_idx(lum: f32, lum_range: &RangeInclusive<f32>) -> usize {
        util::remap(lum, lum_range, &(0.0..=Self::LUM_LEVELS as f32 - 1.0)).round() as usize
    }

    fn find_log_max(array: &Array2D) -> f32 {
        let mut log_max = 0.0;
        for x in 0..array.width() {
            for y in 0..array.height() {
                let elem = array[Index2D::from(x, y)];

                assert!(elem >= 0.0);
                if elem > f32::EPSILON {
                    log_max = (-1.0*(elem.log10())).max(log_max);
                }
            }
        }

        println!("Log-max: {log_max}");
        log_max
    }
}
