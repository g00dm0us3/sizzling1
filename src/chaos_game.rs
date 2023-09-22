use std::ops::Deref;
use crate::ff_repository::affine_transform::AffineIfs;
use crate::modnar::Modnar;
use crate::mutators::{apply_mutator_combination, MutatorConfig};
use crate::util::Point;

pub(crate) struct ChaosGame {
    rnd: Modnar
}

impl ChaosGame {
    pub(crate) fn new() -> Self {
        Self { rnd: Modnar::new_rng() }
    }

    pub(crate) fn run_chaos_game(
        &mut self,
        affine_ifs: &AffineIfs,
        mutators: Option<&[MutatorConfig]>,
        iterations: u32
    ) -> Vec<f32> {
        let mut res = Vec::<f32>::new();
        self.run_chaos_game_(affine_ifs, mutators, iterations, |point| {
            res.push(point.x);
            res.push(point.y);
        });

        res
    }

    fn run_chaos_game_<F>(
        &mut self,
        affine_ifs: &AffineIfs,
        mutators: Option<&[MutatorConfig]>,
        iterations: u32,
        mut point_visitor: F
    )
    where
        F: FnMut(&Point) -> (),
    {
        let mut point: Point = Point::zero();

        for i in 1..=iterations {
            let r: f32 = self.rnd.gen_f32();
            let transform = affine_ifs.transforms
                .iter()
                .find(|t| r <= t.p)
                .expect("Didn't find transform!");

            let mat = &transform.mat;
            point.mul(mat.deref());

            if let Some(mutators) = mutators {
                point = apply_mutator_combination(mutators, &point, mat, &mut self.rnd);
            }

            if i <= 20 {
                continue;
            }
            
            point_visitor(&point);
        }
    }
    // - TODO: refactor.
    pub(crate) fn run_convergence_test(
        rnd: &mut Modnar,
        affine_ifs: &AffineIfs,
        mutators: Option<&[MutatorConfig]>
    ) -> bool {
        let mut nw: Point = Point::new(-1.0, 1.0);
        let mut se = Point::new(1.0, -1.0);

        const ITERATIONS: u8 = 30;

        for i in 1..=ITERATIONS {
            let r: f32 = rnd.gen_f32();

            // select transform at random
            let transform = affine_ifs.transforms
                .iter()
                .find(|t| r <= t.p)
                .expect("Didn't find transform!");

            let mat = &transform.mat;
            nw.mul(mat.deref());
            se.mul(mat.deref());

            if let Some(mutators) = mutators {
                nw = apply_mutator_combination(mutators, &nw, mat, rnd);
                se = apply_mutator_combination(mutators, &se, mat, rnd);
            }
        }

        nw.dst_fast(&se) <= std::f32::consts::SQRT_2 * 0.1
    }
}
