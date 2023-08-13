use crate::ff_repository::model::AffineIfs;
use ndarray::{arr1, Array1};
use rand::prelude::*;

pub(crate) struct ChaosGame;

impl ChaosGame {
    pub(crate) fn run_chaos_game(affine_ifs: &AffineIfs, iterations: u32) -> Vec<f32> {
        let mut res = Vec::<f32>::new();
        Self::run_chaos_game_(affine_ifs, iterations, |point| {
            res.push(point[0]);
            res.push(point[1]);
        });

        res
    }

    fn run_chaos_game_<F>(affine_ifs: &AffineIfs, iterations: u32, mut point_visitor: F)
    where
        F: FnMut(&Array1<f32>) -> (),
    {
        let mut point: Array1<f32> = arr1(&[0.0, 0.0]);

        let mut rng = rand::thread_rng();
        //let normal = Normal::new(0.0, 1.0).expect("Normal distribution");
        //let normal1 = Normal::new(2.0, 1.0).expect("Normal distribution");
        for i in 1..=iterations {
            let r: f32 = rng.gen_range(0.0..=1.0);

            let transform = affine_ifs.transforms.iter().find(|t| r <= t.p).expect("Didn't find transform!");

            let mut three_point = arr1(&[point[0], point[1], 1.0]);
            three_point = transform.mat.dot(&three_point);

            point = three_point;
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
