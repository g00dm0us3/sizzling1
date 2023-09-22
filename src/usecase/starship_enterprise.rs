use crate::alg::big_range_random_cursor::BigRangeRandomCursor;
use crate::ff_repository::presets_repository::PresetsRepository;
use crate::mutators::Mutators;
use crate::ff_repository::mutator_description_service::{MutatorDescription, MutatorDescriptionService};
use crate::modnar::Modnar;
use crate::mutators::Mutators::{Arch, Bent, Blade, Blob, Blur, Bubble, Cosine, Cross, Curl, Cylinder, Diamond, Disc, Ex, Exponential, Eyefish, Fan, Fan2, Fisheye, Gaussian, Handkerchief, Heart, Horseshoe, Hyperbolic, Julia, Julian, Julias, Ngon, Noise, Pdj, Perspective, Pie, Polar, Popcorn, Power, RadianBlur, Rays, Rectangles, Rings, Rings2, Secant, Sinus, Spherical, Spiral, Square, Swirl, Tangent, Twintrian, Waves};

// free search - gen and save images
// randomly traverse the:
// all ifs possibilities
// all mutator combos.
// look for criterion
// 1. Number of non-zero pixels.

pub(crate) struct StarshipEnterprise<'a> {
    presets_repository: &'a PresetsRepository,
    mutators: &'a MutatorDescriptionService,
    mutators_range_cur: BigRangeRandomCursor,
    presets_range_cur: BigRangeRandomCursor,
    rnd: Modnar,
    lsfr: Modnar
}

impl<'a> StarshipEnterprise<'a> {

}

impl MutatorDescription {
    fn into(&self) -> Option<Mutators> {
        match self.enum_id {
            1 => Some(Sinus),
            2 => Some(Spherical),
            3 => Some(Swirl),
            4 => Some(Horseshoe),
            5 => Some(Polar),
            6 => Some(Handkerchief),
            7 => Some(Heart),
            8 => Some(Disc),
            9 => Some(Spiral),
            10 => Some(Hyperbolic),
            11 => Some(Diamond),
            12 => Some(Ex),
            13 => Some(Julia),
            14 => Some(Bent),
            15 => Some(Waves),
            16 => Some(Fisheye),
            17 => Some(Popcorn),
            18 => Some(Exponential),
            19 => Some(Power),
            20 => Some(Cosine),
            21 => Some(Rings),
            22 => Some(Fan),
            23 => Some(Blob { blob_h: self.param("blob_h"), blob_l: self.param("blob_l"), blob_waves: self.param("blob_waves") }),
            24 => Some(Pdj { pdj_a: self.param("pdj_a"), pdj_b: self.param("pdj_b"), pdj_c: self.param("pdj_c"), pdj_d: self.param("pdj_d") }),
            25 => Some(Fan2 { fx: self.param("fx"), fy: self.param("fy") }),
            26 => Some(Rings2 { rings2_val: self.param("rings2_val") }),
            27 => Some(Eyefish),
            28 => Some(Bubble),
            29 => Some(Cylinder),
            30 => Some(Perspective { p1_angle: self.param("p1_angle"), p2_dist: self.param("p2_dist") }),
            31 => Some(Noise),
            32 => Some(Julian { power: self.param("power"), dist: self.param("dist") }),
            33 => Some(Julias { power: self.param("power"), dist: self.param("dist") }),
            34 => Some(Blur),
            35 => Some(Gaussian),
            36 => Some(RadianBlur { angle: self.param("angle"), v36: self.param("v36") }),
            37 => Some(Pie { slices: self.param("slices"), rotation: self.param("rotation"), thickness: self.param("thickness") }),
            38 => Some(Ngon { power: self.param("power"), sides: self.param("sides"), corners: self.param("corners"), circle: self.param("circle") }) ,
            39 => Some(Curl { c1: self.param("c1"), c2: self.param("c2") }),
            40 => Some(Rectangles { rect_x: self.param("rect_x"), rect_y: self.param("rect_y") }),
            41 => Some(Arch { v41: self.param("v41") }),
            42 => Some(Tangent),
            43 => Some(Square),
            44 => Some(Rays { v44: self.param("v44") }),
            45 => Some(Blade { v45: self.param("v45") }),
            46 => Some(Secant { v46: self.param("v46") }),
            47 => Some(Twintrian { v47: self.param("v47") }),
            48 => Some(Cross),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ff_repository::mutator_description_service::{MutatorDescription, MutatorDescriptionService};

    #[test]
    fn test_conversion() {
        let muts = MutatorDescriptionService::load("")
            .expect("DB cannot be loaded!");

        muts.as_ref().into_iter().for_each(|desc| { desc.into().expect("Ooops!"); });
    }
}