use std::ops::RangeInclusive;

use crate::{statistics::samples::BivariateSample, ds::array_2d::{Index2D, Array2D}};
use crate::statistics::samples::Axis;
use super::samples::RandomVec2D;
use crate::util::remap;

pub(crate) struct DensityEstimator2D {
    samples: BivariateSample
}

impl DensityEstimator2D {
    pub(crate) fn from(data: &[f32]) -> Self {
        Self { samples: BivariateSample::from(data) }
    }
}

impl DensityEstimator2D {
    // `width`- number of X-axis bins [0..width-1]
    // `height` - number of Y-axis bins [0..height-1]
    pub(crate) fn histogram(&self, width: usize, height: usize) -> Array2D {
        let mut res = Array2D::new(width, height);

        let grid_x_range = 0.0..=((width as f32) - 1.0);
        let grid_y_range = 0.0..=((height as f32) - 1.0);

        for sample in self.samples.iter() {
            let remapped_x = remap(sample[Axis::x], self.samples.region().x_range(), &grid_x_range);
            let remapped_y = remap(sample[Axis::y], self.samples.region().y_range(), &grid_y_range);

            let index = Index2D::from(remapped_x.round() as usize, remapped_y.round() as usize);
            res[index] += 1.0 / (self.samples.len() as f32);
        }

        res
    }
}

impl DensityEstimator2D {
    const ADAPTIVE_APPROX_STEPS: u8 = 2;

    pub(crate) fn kde_const(&self, width: usize, height: usize, h: f32) -> Array2D { self.kde(width, height, |_| h) }

    pub(crate) fn kde_adapt(&self, width: usize, height: usize) -> Array2D {
        let mut pilot = self.histogram(width, height);

        for i in 0..Self::ADAPTIVE_APPROX_STEPS {
            println!("Running {i} estimate for adaptive KDE...");
            let curr = self.kde(width, height, |r_vec| {
                let pilot_est = pilot[Index2D::from(r_vec[Axis::x].round() as usize, r_vec[Axis::y].round() as usize)];

                if pilot_est <= f32::EPSILON {
                    1.0
                } else {
                    // - TODO: this can be tinkered with, see that Montecarlo article.
                    20.0 / ((self.samples.len() as f32 * pilot_est).sqrt())
                }
            });
            pilot = curr
        }

        pilot
    }

    fn kde<F>(&self, width: usize, height: usize, get_h: F) -> Array2D where F: Fn(&RandomVec2D)->f32 {
        let mut res = Array2D::new(width, height);

        let grid_x_range = 0.0..=((width as f32) - 1.0);
        let grid_y_range = 0.0..=((height as f32) - 1.0);

        let mut coords = vec![Index2D::from(0, 0); width * height];

        /* 0--1--2--3--4--5
           |  |  |  |  |  |
           0--1--2--3--4--5
           |  |  |  |  |  |
           1--1--2--3--4--5

           Grid. Centers are integer coordinates (aka. "remapped rounded (aka binned) samples")
        */

        for sample in self.samples.iter() {
            let remapped_x = remap(sample[Axis::x], self.samples.region().x_range(), &grid_x_range);
            let remapped_y = remap(sample[Axis::y], self.samples.region().y_range(), &grid_y_range);
            let remapped_sample = RandomVec2D::from(remapped_x, remapped_y);
            let h = get_h(&remapped_sample);
            let cell_count = Self::cells_to_inc_around(
                remapped_sample,
                h,
                &mut coords,
                &grid_x_range,
                &grid_y_range
            );
            
            assert!(h > 0.0);
            let normalization_c = 1.0 / (self.samples.len() as f32 * h * h);

            coords[0..cell_count].iter().for_each(|int_sample| {
                let kernel_val_x = Self::epanechnikov_kernel(Self::distance(remapped_x, int_sample.x() as f32, h));
                let kernel_val_y = Self::epanechnikov_kernel(Self::distance(remapped_y, int_sample.y() as f32, h));

                let increment = normalization_c * kernel_val_x * kernel_val_y;
                res[*int_sample] += increment;
            });
        }

        res
    }

    #[inline]
    fn distance(sample: f32, grid_point: f32, h: f32) -> f32 { (sample - grid_point).abs() / h }

    // For kernels with compact support (i.e. Epanechnikov)
    // will return all of the grid cells, which should be
    // incremented with a non-zero value.
    // For kernels without compact support (i.e. Gaussian) - dunno (qqq-quad tree! hm?)
    fn cells_to_inc_around(
        remapped_s: RandomVec2D,
        h: f32,
        coords: &mut Vec<Index2D>,
        range_x: &RangeInclusive<f32>,
        range_y: &RangeInclusive<f32>
    ) -> usize {
        // - TODO: can we returns a contiguous memory run, which can then be iterated ??
        // Perhaps Hilbert curve would do us some good - by selecting a range of x and y
        // we can just get a contiguous (this is important) subsequence of the nodes?

        let h = h.ceil();
        // ceil - if we even touch some grid cell a little, we need to consider it.
        let max_x = (remapped_s[Axis::x] + h).min(*range_x.end()).ceil() as usize; // f#^!ck typesafety
        let min_x = (remapped_s[Axis::x] - h).max(*range_x.start()).ceil() as usize;
        let max_y = (remapped_s[Axis::y] + h).min(*range_y.end()).ceil() as usize;

        let min_y = (remapped_s[Axis::y] - h).max(*range_y.start()).ceil() as usize;

        let mut count = 0;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                coords[count] = Index2D::from(x, y);
                count += 1;
            }
        }

        count
    }

    #[inline]
    fn epanechnikov_kernel(u: f32) -> f32 { (3.0/4.0 *(1.0 - u*u)).max(0.0) }
}

#[cfg(test)]
mod tests {
    use crate::{statistics::samples::RandomVec2D, ds::array_2d::Index2D};

    use super::DensityEstimator2D;

    #[test]
    fn test_cells_to_inc_around_top_left_corner() {
        let mut res = vec![Index2D::from(0, 0); 4];
        let count = DensityEstimator2D::cells_to_inc_around(
            RandomVec2D::from(0.0, 0.0),
            1.0,
            &mut res,
            &(0.0..=1.0),
            &(0.0..=1.0)
        );

        assert_eq!(count, 4);

        let expected = vec![Index2D::from(0, 0), Index2D::from(0, 1), Index2D::from(1, 0), Index2D::from(1, 1)];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_cells_to_inc_around_bottom_right_corner() {
        let mut res = vec![Index2D::from(0, 0); 9];
        let count = DensityEstimator2D::cells_to_inc_around(
            RandomVec2D::from(2.0, 2.0),
            1.0,
            &mut res,
            &(0.0..=2.0),
            &(0.0..=2.0)
        );

        assert_eq!(count, 4);

        let expected = vec![Index2D::from(1, 1), Index2D::from(1, 2), Index2D::from(2, 1), Index2D::from(2, 2)];
        assert_eq!(res[0..count], expected);
    }

    // - TODO: test. something that should integrate to unity, and be closer to distribution then original.
}