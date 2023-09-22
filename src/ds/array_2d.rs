use std::ops::{Index, IndexMut};
use std::slice::IterMut;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MultiIndex<const N: usize> {
    axis: [usize; N]
}

impl<const N: usize> MultiIndex<N> {
    // zigzag indexing style
    // - TODO: implement Hilbert curve.
    fn lin_index(&self, axis_dimensions: [usize; N]) -> usize {
        assert!(N > 0);
        // I sure hope these abstractions are zero-cost.
        // in 2d lin_idx = width*y + x
        let first_part: usize = self.axis
        .into_iter()
        .rev()
        .take(N-1)
        // zip: y
        // width, height 
        .zip(axis_dimensions.into_iter())
        .map(|pair| pair.0 * pair.1)
        .sum();

        return first_part + self.axis[0];
    }

    fn next(&mut self) {
        for i in 0..self.axis.len() {
            self.axis[i] = self.axis[i] + 1;
        }
    }
}

impl MultiIndex<2> {
    pub(crate) const fn from(x: usize, y: usize) -> Self { Self { axis: [x, y]  } }

    pub(crate) fn x(&self) -> usize { self.axis[0] }
    pub(crate) fn y(&self) -> usize { self.axis[1] }
}

pub(crate) struct Iter {
    curr_index: Index2D,
    axis_dimensions: [usize; 2]
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {

        if self.curr_index.axis[0] >= self.axis_dimensions[0] {
            return None;
        }

        if self.curr_index.axis[1] >= self.axis_dimensions[1] {
            return None;
        }

        let res = self.curr_index.lin_index(self.axis_dimensions);
        self.curr_index.next();

        Some(res)
    }
} 

pub(crate) type Index2D = MultiIndex<2>;

pub(crate) struct Array<const N: usize> {
    array: Vec<f32>,
    dims:[usize; N]
}

pub(crate) type Array2D = Array<2>;

impl Array<2> {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            array: vec![0.0; width * height],
            dims: [width, height]
        }
    }

    pub(crate) fn width(&self) -> usize { self.dims[0] }
    pub(crate) fn height(&self) -> usize { self.dims[1] }

    pub(crate) fn non_zero_count(&self) -> usize { self.array.iter().filter(|val| **val != 0.0).count() }
}

impl Index<Index2D> for Array<2> {
    type Output = f32;

    fn index(&self, index: Index2D) -> &Self::Output {
        let lin_index = index.lin_index(self.dims);
        assert!(lin_index < self.array.len());
        &self.array[lin_index]
    }
}

impl IndexMut<Index2D> for Array2D {
    fn index_mut(&mut self, index: Index2D) -> &mut Self::Output {
        let lin_index = index.lin_index(self.dims);
        &mut self.array[lin_index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lin_index() {
        let index = Index2D::from(2, 2);

        let mut lin = index.lin_index([3,3]);

        assert_eq!(lin, 8);

        lin = index.lin_index([4,3]);

        assert_eq!(lin, 10);
    }

    #[test]
    fn test_array_2d_write_read() {
        let mut array = Array2D::new(3, 3);

        for a in 1..=3 {
            for b in 1..=3 {
                array[Index2D::from(a - 1, b - 1)] = (a * b) as f32;
            }
        }

        for a in 1..=3 {
            for b in 1..=3 {
                assert_eq!(array[Index2D::from(a - 1, b - 1)], (a * b) as f32);
            }
        }

    }
}