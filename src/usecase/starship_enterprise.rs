use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Deref;
use std::time::Instant;
use crate::alg::big_range_random_cursor::BigRangeRandomCursor;
use crate::alg::combinations::Combinations;
use crate::chaos_game::ChaosGame;
use crate::ff_repository::affine_transform::{AffineMat, AffineTransform};
use crate::ff_repository::presets_repository::PresetsRepository;
use crate::mutators::Mutators;
use crate::ff_repository::mutator_description_service::{MutatorDescription, MutatorDescriptionService};
use crate::frac_render::RgbRenderer;
use crate::mutators::Mutators::{Arch, Bent, Blade, Blob, Blur, Bubble, Cosine, Cross, Curl, Cylinder, Diamond, Disc, Ex, Exponential, Eyefish, Fan, Fan2, Fisheye, Gaussian, Handkerchief, Heart, Horseshoe, Hyperbolic, Julia, Julian, Julias, Ngon, Noise, Pdj, Perspective, Pie, Polar, Popcorn, Power, RadianBlur, Rays, Rectangles, Rings, Rings2, Secant, Sinus, Spherical, Spiral, Square, Swirl, Tangent, Twintrian, Waves};
use crate::statistics::grid_density::DensityEstimator2D;

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
    chaos_game: ChaosGame,
    combinations: Combinations
}

impl<'a> StarshipEnterprise<'a> {
    pub(crate) fn new(
        presets: &'a PresetsRepository,
        mutators: &'a MutatorDescriptionService
    ) -> Self {
        let mut combinations = Combinations::new();
        let total_presets_comp = combinations.combinations(presets.flatted.len() as u8, 4);
        println!("Total combinations {total_presets_comp}");
        Self {
            presets_repository: presets,
            mutators,
            // - TODO: range over total number of combinations from n by k.
            mutators_range_cur: BigRangeRandomCursor::new_clean(1..=mutators.as_ref().len() as u64),
            presets_range_cur: BigRangeRandomCursor::new_clean(1..=total_presets_comp),
            chaos_game: ChaosGame::new(),
            combinations: Combinations::new()
        }
    }

    pub(crate) fn roll_dice_presets(
        &mut self,
        path_to_samples: &str,
        total_img: u16
    ) {
        let mut img_generated = 0u16;
        let mut discarded = 0u16;
        while img_generated < total_img {
            if let Some(perm_rank) = self.presets_range_cur.next() {
                //println!("Discarded {discarded}");
                let comb: HashSet<usize> = self.combinations
                    .unrank(perm_rank, self.presets_repository.flatted.len() as u8, 4)
                    .into_iter()
                    .map(|e| (e - 1) as usize)
                    .collect();

                let mut ifs: Vec<AffineTransform> = self.presets_repository
                    .flatted
                    .iter()
                    .enumerate()
                    .map(|e| {
                        if comb.contains(&e.0) {
                            Some(e.1.clone())
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect();

                Self::sort_ifs(&mut ifs);
                Self::set_cumulative_probs(&mut ifs);


                if self.chaos_game.run_convergence_test(&ifs, None) {

                    let now = Instant::now();
                    let samples = self
                        .chaos_game
                        .run_chaos_game(&ifs, None, 400_000);
                    let density = DensityEstimator2D::new(&samples).histogram(256, 256);

                    let elapsed = now.elapsed();
                    println!("Compute density in {}", elapsed.as_secs_f32());

                    let min_fill = 0.1 * 256.0 * 256.0 as f32;

                    if (density.non_zero_count() as f32) < min_fill {
                        discarded += 1;
                        continue;
                    }

                    let img = RgbRenderer::img_bw_simple(&density);

                    img.save(&format!("{path_to_samples}\\{perm_rank}.png")).unwrap();

                    img_generated += 1;
                    println!("Presets {img_generated} out of {total_img}");
                } else {
                    discarded += 1;
                }

                continue
            }
        }
    }

    fn sort_ifs(ifs: &mut Vec<AffineTransform>) {
        // - TODO: welp this pretty much erodes our need for dependency on ndarray
        fn det(mat: &AffineMat) -> f32 {
            mat[[0,0]]*mat[[1,1]] - mat[[0,1]]*mat[[1,0]]
        }

        let total_det: f32 = ifs
            .iter()
            .map(|e| { return det(&e.mat).abs() })
            .sum();

        ifs.iter_mut().for_each(|t| {
            t.p = det(&t.mat).abs()/total_det;
        });
        ifs.sort_by(|rhs, lhs| {
            return if rhs.p > lhs.p {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        });
    }

    fn set_cumulative_probs(ifs: &mut Vec<AffineTransform>) {
        let mut cum_prob = 0.0;

        // - TODO: is there an idiom for this??
        let last_idx = ifs.len() - 1;
        for (idx, transform) in ifs.iter_mut().enumerate() {
            cum_prob += transform.p;
            transform.p = cum_prob;

            if idx == last_idx { transform.p = 1.0; }
        }
    }
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
    use crate::ff_repository::mutator_description_service::MutatorDescriptionService;

    #[test]
    fn test_conversion() {
        let muts = MutatorDescriptionService::load("")
            .expect("DB cannot be loaded!");

        muts.as_ref().into_iter().for_each(|desc| { desc.into().expect("Ooops!"); });
    }
}