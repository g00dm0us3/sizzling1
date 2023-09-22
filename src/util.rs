use std::ops::{RangeInclusive, Sub, Deref};
use ndarray::{Array1, Array2, arr1};


#[derive(Clone)]
pub(crate) struct Point {
    // - TODO: Probably should make this box. for some reason it cant be copied.
    base: Array1<f32>,
    pub(crate) x: f32,
    pub(crate) y: f32
}

impl Point {
    pub(crate) fn new(x: f32, y: f32) ->  Self {
        Self { base: arr1(&[x, y, 1.0]), x: x, y: y }
    }
    pub(crate) fn zero() -> Self { Self { base: arr1(&[0.0, 0.0, 1.0]), x: 0.0, y: 0.0 } }

    pub(crate) fn mul(&mut self, mat: &Array2<f32>) {
        self.base = mat.dot(&self.base);
        self.x = self.base[0];
        self.y = self.base[1];
    }
}

// - NOTE: Deref, via deref coercion in many situations 
// allows to treat our custom type as original + some
impl Deref for Point {
    type Target = Array1<f32>;

    fn deref(&self) -> &Self::Target { &self.base }
}

pub(crate) trait Len<Idx>
where
    Idx: Sub,
{
    fn len(&self) -> Idx;
}

impl Len<f32> for RangeInclusive<f32> {
    fn len(&self) -> f32 {
        return (self.end() - self.start()).abs();
    }
}

impl Len<u64> for RangeInclusive<u64> {
    fn len(&self) -> u64 {
        return self.end().max(self.start()) - self.start().min(self.end());
    }
}

pub(crate) fn remap(val: f32, old_range: &RangeInclusive<f32>, new_range: &RangeInclusive<f32>) -> f32 {
    let old_len = old_range.len().max(f32::EPSILON);
    let new_len = new_range.len();

    let res = ((val - old_range.start()) * new_len / old_len) + new_range.start();

    res.max(*new_range.start()).min(*new_range.end())
}



#[cfg(test)]
mod test {
    use ndarray::Array;
    use crate::util::Point;
    use super::remap;

    #[test]
    fn test_remap() {
        let res = remap(0.5, &(0.0..=1.0), &(0.0..=255.0));

        assert_eq!(res, 127.5);
    }

    #[test]
    fn test_point_mul() {
        let affine_mat: ndarray::Array2<f32> = ndarray::arr2(&[[1.0, 0.0, 1.0], [0.0, 0.0, 1.0]]);
        let mut vec = Point::zero();

        vec.mul(&affine_mat);
    }
}
