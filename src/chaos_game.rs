use crate::ds::aff_ifs::AffIfs;
use crate::ds::ifs_transform::IfsTransform;
use crate::ds::point::Point;
use crate::modnar::Modnar;
use crate::mutators::{apply_mutator_combination, MutatorConfig};

pub(crate) struct ChaosGame {
    rnd: Modnar,
    lsfr: Modnar
}

pub(crate) trait AffineTransformProvider {
    fn find_transform(&self, prob: f32) -> Option<&IfsTransform>;
}

impl AffineTransformProvider for Vec<IfsTransform> {
    fn find_transform(&self, prob: f32) -> Option<&IfsTransform> {
        self.iter()
            .find(|t| prob <= t.p)
    }
}

impl AffineTransformProvider for &AffIfs {
    fn find_transform(&self, prob: f32) -> Option<&IfsTransform> {
        self.transforms
            .iter()
            .find(|t| prob <= t.p)
    }
}

impl ChaosGame {
    pub(crate) fn new() -> Self {
        Self { rnd: Modnar::new_rng(),lsfr:  Modnar::new_lsfr(7) }
    }

    pub(crate) fn run_chaos_game(
        &mut self,
        aff_t_provider: &impl AffineTransformProvider,
        mutators: Option<&[MutatorConfig]>,
        iterations: u32
    ) -> Vec<f32> {
        let mut res = Vec::<f32>::new();
        self.run_chaos_game_(aff_t_provider, mutators, iterations, |point| {
            res.push(point.x);
            res.push(point.y);
        });

        res
    }

    fn run_chaos_game_<F>(
        &mut self,
        aff_t_provider: &impl AffineTransformProvider,
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
            let transform = aff_t_provider
                .find_transform(r)
                .expect("Didn't find transform!");

            let mat = &transform.mat;
            point.transform(mat);

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
        &mut self,
        aff_t_provider: &impl AffineTransformProvider,
        mutators: Option<&[MutatorConfig]>
    ) -> bool {
        let mut nw = Point::new(-1.0, 1.0);
        let mut se = Point::new(1.0, -1.0);

        const ITERATIONS: u8 = 30;

        for i in 1..=ITERATIONS {
            let r: f32 = self.lsfr.gen_f32();

            // select transform at random
            let transform = aff_t_provider
                .find_transform(r)
                .expect("Didn't find transform!");

            let mat = &transform.mat;
            nw.transform(mat);
            se.transform(mat);

            if let Some(mutators) = mutators {
                nw = apply_mutator_combination(mutators, &nw, mat, &mut self.rnd);
                se = apply_mutator_combination(mutators, &se, mat, &mut self.rnd);
            }
        }

        nw.dst_fast(&se) <= std::f32::consts::SQRT_2 * 0.1
    }
}