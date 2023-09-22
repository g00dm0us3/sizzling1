use std::ops::Deref;
use crate::ff_repository::model::AffineIfs;
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
}
