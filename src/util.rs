use std::ops::{RangeInclusive, Sub};

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
    use super::remap;

    #[test]
    fn test_remap() {
        let res = remap(0.5, &(0.0..=1.0), &(0.0..=255.0));

        assert_eq!(res, 127.5);
    }
}
