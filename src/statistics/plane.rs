use std::ops::RangeInclusive;

type AxisRange = RangeInclusive<f32>;

pub(crate) struct Range2D {
    x: AxisRange,
    y: AxisRange
}

impl Range2D {
    pub(crate) fn new(x: AxisRange, y: AxisRange) -> Self {
        return Self {
            x: x,
            y: y
        }
    }

    pub(crate) fn x_range(&self) -> &AxisRange { &self.x }

    pub(crate) fn y_range(&self) -> &AxisRange { &self.y }
}