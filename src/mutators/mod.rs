mod mutator_lib;

use crate::{util::Point, modnar::Modnar};

fn call(mutator: Mutators, point: &mut Point, params: &[f32]) {
    match mutator {
        Mutators::Lin => todo!(),
        Mutators::Sinus => todo!(),
        Mutators::Spherical => todo!(),
        Mutators::Swirl => todo!(),
        Mutators::Horseshoe => todo!(),
        Mutators::Polar => todo!(),
        Mutators::Handkerchief => todo!(),
        Mutators::Heart => todo!(),
        Mutators::Disc => todo!(),
        Mutators::Spiral => todo!(),
        Mutators::Hyperbolic => todo!(),
        Mutators::Diamond => todo!(),
        Mutators::Ex => todo!(),
        Mutators::Julia => todo!(),
        Mutators::Bent => todo!(),
        Mutators::Waves => todo!(),
        Mutators::Fisheye => todo!(),
        Mutators::Popcorn => todo!(),
        Mutators::Exponential => todo!(),
        Mutators::Power => todo!(),
        Mutators::Cosine => todo!(),
        Mutators::Rings => todo!(),
        Mutators::Fan => todo!(),
        Mutators::Blob => todo!(),
        Mutators::Pdj => todo!(),
        Mutators::Fan2 => todo!(),
        Mutators::Rings2 => todo!(),
        Mutators::Eyefish => todo!(),
        Mutators::Bubble => todo!(),
        Mutators::Cylinder => todo!(),
        Mutators::Perspective => todo!(),
        Mutators::Noise => todo!(),
        Mutators::Julian => todo!(),
        Mutators::Julias => todo!(),
        Mutators::Blur => todo!(),
        Mutators::Gaussian => todo!(),
        Mutators::RadianBlur => todo!(),
        Mutators::Pie => todo!(),
        Mutators::Ngon => todo!(),
        Mutators::Curl => todo!(),
        Mutators::Rectangles => todo!(),
        Mutators::Arch => todo!(),
        Mutators::Tangent => todo!(),
        Mutators::Square => todo!(),
        Mutators::Rays => todo!(),
        Mutators::Blade => todo!(),
        Mutators::Secant => todo!(),
        Mutators::Twintrian => todo!(),
        Mutators::Cross => todo!(),
    }
}

pub(crate) struct MutatorConfig<'a> {
    weight: f32,
    mutator: Mutators,
    params:&'a [f32]
}

pub(crate) fn apply_mutator_combination(
    modnar: &mut Modnar,
    mutators: &[MutatorConfig],
    point: &mut Point
) {
    mutators.iter().for_each(|mut_config| {
        call(
            mut_config.mutator,
            point,
            mut_config.params
        )
    })
}

// same as [repr(C, u8)], u8 concerns discriminant vals, C concerns size and alignment of params.
// https://doc.rust-lang.org/reference/type-layout.html
#[repr(u8)]
#[derive(Copy, Clone)]
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