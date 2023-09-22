mod mutator_lib;

use std::ops::RangeInclusive;
use crate::{util::Point, modnar::Modnar};
use crate::ff_repository::affine_transform::AffineMat;
use crate::mutators::mutator_lib::{bent, blob, cosine, diamond, disc, ex, exponential, fan, fisheye, handkerchief, heart, horseshoe, hyperbolic, julia, polar, popcorn, power, rings, sinus, spherical, spiral, swirl, waves, pdj, fan2, rings2, eyefish, bubble, cylinder, perspective, noise, julian, julias, blur, gaussian, radian_blur, pie, ngon, curl, rectangles, arch, tangent, square, rays, blade, secant, twintrian, cross};

pub(crate) struct MutatorConfig{
    weight: f32,
    mutator: Mutators
}

impl MutatorConfig {
    pub(crate) fn new(weight: f32, mutator: Mutators) -> Self {
        Self { weight: weight, mutator: mutator }
    }
}

pub(crate) fn apply_mutator_combination(
    mutators: &[MutatorConfig],
    point: &Point,
    mat: &AffineMat,
    rnd: &mut Modnar
) -> Point {
    mutators.iter().fold(Point::zero(), |acc, mutator| {
        let app_res = call(mutator.mutator, point, mat, rnd);

       Point::new(
           // - TODO: eh, operator overloading.
           acc.x + mutator.weight*app_res.x,
           acc.y + mutator.weight*app_res.y
       )
    })
}


pub(crate) const ALL_MUTATOR_DISCRIMINANTS: RangeInclusive<u8> = 1..=48;
// same as [repr(C, u8)], u8 concerns discriminant vals, C concerns size and alignment of params.
// https://doc.rust-lang.org/reference/type-layout.html
#[repr(u8)]
#[allow(unused)]
#[derive(Copy, Clone)]
pub(crate) enum Mutators {
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
    Blob { blob_h: f32, blob_l: f32, blob_waves: f32 } = 23,
    Pdj { pdj_a: f32, pdj_b: f32, pdj_c: f32, pdj_d: f32 } = 24,
    Fan2 { fx: f32, fy: f32 } = 25,
    Rings2 { rings2_val: f32 } = 26,
    Eyefish = 27,
    Bubble = 28,
    Cylinder = 29,
    Perspective { p1_angle: f32, p2_dist: f32 } = 30,
    Noise = 31,
    Julian { power: f32, dist: f32 } = 32,
    Julias { power: f32, dist: f32 } = 33,
    Blur = 34,
    Gaussian = 35,
    RadianBlur { angle: f32, v36: f32 } = 36,
    Pie { slices: f32, rotation: f32, thickness: f32 } = 37,
    Ngon { power: f32, sides: f32, corners: f32, circle: f32 } = 38,
    Curl { c1: f32, c2: f32 } = 39,
    Rectangles { rect_x: f32, rect_y: f32 } = 40,
    Arch { v41: f32 } = 41,
    Tangent = 42,
    Square = 43,
    Rays { v44: f32 } = 44,
    Blade { v45: f32 } = 45,
    Secant { v46: f32 } = 46,
    Twintrian { v47: f32 } = 47,
    Cross = 48
}

fn call(
    mutator: Mutators,
    p: &Point,
    mat: &AffineMat,
    rnd: &mut Modnar
) -> Point {
    match mutator {
        Mutators::Sinus => sinus(p),
        Mutators::Spherical => spherical(p),
        Mutators::Swirl => swirl(p),
        Mutators::Horseshoe => horseshoe(p),
        Mutators::Polar => polar(p),
        Mutators::Handkerchief => handkerchief(p),
        Mutators::Heart => heart(p),
        Mutators::Disc => disc(p),
        Mutators::Spiral => spiral(p),
        Mutators::Hyperbolic => hyperbolic(p),
        Mutators::Diamond => diamond(p),
        Mutators::Ex => ex(p),
        Mutators::Julia => julia(p, rnd),
        Mutators::Bent => bent(p),
        // - TODO: mutators, dependent on transform applied. has something to it.
        Mutators::Waves => waves(p, mat.b(), mat.c(), mat.e(), mat.f()),
        Mutators::Fisheye => fisheye(p),
        Mutators::Popcorn => popcorn(p, mat.c(), mat.f()),
        Mutators::Exponential => exponential(p),
        Mutators::Power => power(p),
        Mutators::Cosine => cosine(p),
        Mutators::Rings => rings(p, mat.c()),
        Mutators::Fan => fan(p, mat.c(), mat.f()),
        Mutators::Blob { blob_h, blob_l, blob_waves } => blob(p, blob_h, blob_l, blob_waves),
        Mutators::Pdj { pdj_a, pdj_b, pdj_c, pdj_d } => pdj(p, pdj_a, pdj_b, pdj_c, pdj_d),
        Mutators::Fan2 { fx, fy } => fan2(p, fx, fy),
        Mutators::Rings2 { rings2_val } => rings2(p, rings2_val),
        Mutators::Eyefish => eyefish(p),
        Mutators::Bubble => bubble(p),
        Mutators::Cylinder => cylinder(p),
        Mutators::Perspective { p1_angle, p2_dist } => perspective(p, p1_angle, p2_dist),
        Mutators::Noise => noise(p, rnd),
        Mutators::Julian { power, dist } => julian(p, rnd, power, dist),
        Mutators::Julias { power, dist } => julias(p, rnd, power, dist),
        Mutators::Blur => blur(p, rnd),
        Mutators::Gaussian => gaussian(p, rnd),
        Mutators::RadianBlur { angle, v36 } => radian_blur(p, rnd, angle, v36),
        Mutators::Pie { slices, rotation, thickness } => pie(p, rnd, slices,rotation, thickness),
        Mutators::Ngon { power, sides, corners, circle } => ngon(p, power, sides, corners, circle),
        Mutators::Curl { c1, c2 } => curl(p, c1, c2),
        Mutators::Rectangles { rect_x, rect_y } => rectangles(p, rect_x, rect_y),
        Mutators::Arch { v41 } => arch(p, rnd, v41),
        Mutators::Tangent => tangent(p),
        Mutators::Square => square(p, rnd),
        Mutators::Rays { v44 } => rays(p, rnd, v44),
        Mutators::Blade { v45 } => blade(p, rnd, v45),
        Mutators::Secant { v46 } => secant(p, v46),
        Mutators::Twintrian { v47 } => twintrian(p, rnd, v47),
        Mutators::Cross => cross(p),
    }
}