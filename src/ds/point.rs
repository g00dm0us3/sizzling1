use crate::ds::affine_mat::AffineMat;

#[derive(Clone)]
pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32
}

impl Point {
    pub(crate) fn new(x: f32, y: f32) ->  Self {
        Self { x, y }
    }
    pub(crate) fn zero() -> Self { Self { x: 0.0, y: 0.0 } }

    pub(crate) fn transform(&mut self, mat: &AffineMat) {
        self.x = self.x*mat.a + self.y*mat.b + mat.e;
        self.y = self.x*mat.c + self.y*mat.d + mat.f;
    }

    pub(crate) fn dst_fast(&self, rhs: &Point) -> f32 {
        // sqrt ( |x1 - x2| + |y1 - y2| )
        // if both are small, it will be small.
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()).sqrt()
    }

    pub(crate) fn dot(&self, rhs: &Point) -> f32 {
        self.x*rhs.x + self.y*rhs.y
    }

    pub(crate) fn len(&self) -> f32 {
        self.dot(self)
    }
}