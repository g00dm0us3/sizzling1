
use std::ops::Index;
use crate::{statistics::plane::Range2D, util::Point};

#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub(crate) enum Axis { x, y }

pub(crate) type RandomVec2D = RandomVec<2>;

pub(crate) struct BivariateSample {
    samples: Vec<RandomVec2D>,
    region: Range2D
}

pub(crate) struct RandomVec<const U: usize> {
    array: [f32; U],
}

impl RandomVec<2> {
    pub(crate) const fn from(x: f32, y: f32) -> Self { Self { array: [x, y] } }
}

impl Index<Axis> for RandomVec<2> {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::x => &self.array[0],
            Axis::y => &self.array[1]
        }
    }
}

impl BivariateSample {
    pub(crate) fn from(data: &[f32]) -> Self {
        assert!(data.len() % 2 == 0 && data.len() != 0);

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y =  f32::MIN;

        let samples: Vec<RandomVec2D> = data
            .chunks(2)
            .map(|node|  {
                min_x = node[0].min(min_x);
                min_y = node[1].min(min_y);
                max_x = node[0].max(max_x);
                max_y = node[1].max(max_y);
                RandomVec2D::from(node[0], node[1]) 
            })
            .collect();
         Self { 
            samples: samples,
            region: Range2D::new(min_x..=max_x, min_y..=max_y) 
        }
    }

    pub(crate) fn len(&self) -> usize { self.samples.len() }

    pub(crate) fn region(&self) -> &Range2D { &self.region }

}

pub(crate) struct Iter<'a> {
    samples_iterator: std::slice::Iter<'a, RandomVec<2>>
}

// ref. to item exists as long as the iterator.
// but - can't deduce lifetime to be longer.
impl<'a> Iterator for Iter<'a> {
    type Item = &'a RandomVec2D;

    fn next(&mut self) -> Option<Self::Item> {
        return self.samples_iterator.next();
    }
}

impl BivariateSample {
    pub(crate) fn iter(&self) -> Iter {
        Iter { samples_iterator: self.samples.iter() } 
    }
}