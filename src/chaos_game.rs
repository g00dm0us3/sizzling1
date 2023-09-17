use crate::ff_repository::model::AffineIfs;
use ndarray::{arr1, Array1};
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
            res.push(point[0]);
            res.push(point[1]);
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
        F: FnMut(&Array1<f32>) -> (),
    {
        let mut point: Array1<f32> = arr1(&[0.0, 0.0]);

        //let normal = Normal::new(0.0, 1.0).expect("Normal distribution");
        //let normal1 = Normal::new(2.0, 1.0).expect("Normal distribution");
        for i in 1..=iterations {
            let r: f32 = self.rnd.gen_f32();
            let transform = affine_ifs.transforms.iter().find(|t| r <= t.p).expect("Didn't find transform!");

            let mut three_point = arr1(&[point[0], point[1], 1.0]);
            three_point = transform.mat.dot(&three_point);

            if let Some(mutators) = mutators {
                let p = Point::new(three_point[0], three_point[1]);
                let rnd = &mut self.rnd;
                let res = apply_mutator_combination(mutators, &p, rnd);

                point = arr1(&[res.x, res.y, 1.0]);
            } else {
                point = three_point;
            }
            if i <= 20 {
                continue;
            }
            
            point_visitor(&point);
            //let x = normal.sample(&mut rng) + normal1.sample(&mut rng);
            //let y = normal.sample(&mut rng) + normal1.sample(&mut rng);

            //point_visitor(&arr1(&[x, y]));
        }
    }
}
