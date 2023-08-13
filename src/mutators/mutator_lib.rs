use crate::util::Point;
use std::ops::Deref;

trait PointOps {
    fn r_f(&self) -> f32;
    fn rsq_f(&self) -> f32;
    fn theta_f(&self) -> f32;
    fn phi_f(&self) -> f32;
}

impl PointOps for Point {
    fn r_f(&self) -> f32 {
        self.dot(self.deref()).sqrt()
    }
    fn rsq_f(&self) -> f32 {
        self.dot(self.deref())
    }
    fn theta_f(&self) -> f32 {
        (self.x / (self.y + f32::EPSILON)).atan()
    }
    fn phi_f(&self) -> f32 {
        (self.y / (self.x + f32::EPSILON)).atan()
    }
}

fn sinus(p: &Point) -> Point {
    Point::new(p.x.sin(), p.y.sin())
}

// same as [repr(C, u8)], u8 concerns discriminant vals, C concerns size and alignment of params.
// https://doc.rust-lang.org/reference/type-layout.html
#[repr(u8)]
enum Mutators {
    Lin = 0,
    Sinus = 1,
    Spherical = 2,
    Swirl = 3,
    Horseshoe = 4,
    Polar = 5,
    Handkerchief = 6,
    Heart = 7,
    Disc = 8,
    Spiral = 9,
    Hyperbolic = 10,
    Diamond = 11,
    Ex = 12,
    Julia = 13,
    Bent = 14,
    Waves = 15,
    Fisheye = 16,
    Popcorn = 17,
    Exponential = 18,
    Power = 19,
    Cosine = 20,
    Rings = 21,
    Fan = 22,
    Blob = 23,
    Pdj = 24,
    Fan2 = 25,
    Rings2 = 26,
    Eyefish = 27,
    Bubble = 28,
    Cylinder = 29,
    Perspective = 30,
    Noise = 31,
    Julian = 32,
    Julias = 33,
    Blur = 34,
    Gaussian = 35,
    RadianBlur = 36,
    Pie = 37,
    Ngon = 38,
    Curl = 39,
    Rectangles = 40,
    Arch = 41,
    Tangent = 42,
    Square = 43,
    Rays = 44,
    Blade = 45,
    Secant = 46,
    Twintrian = 47,
    Cross = 48
}


// from array (or map??), if map than named to named for construction
