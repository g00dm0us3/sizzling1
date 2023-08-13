mod alg;
mod chaos_game;
mod ff_repository;
mod statistics;
mod util;
mod ds;
mod frac_render;
mod mutators;

use std::time::Instant;

use ff_repository::service::PresetService;
use statistics::grid_density::DensityEstimator2D;

use chaos_game::ChaosGame;

use crate::{frac_render::RgbRenderer, ds::array_2d::Index2D};

// grids, which are too small don't yield detailed results (all samples endup in the same bins)
// - TODO: qaud-tree to grid?
// - TODO: supersampling should probably be a thing.
fn main() {
    // - TODO: erase before commit.
    let presets = PresetService::load("ifs_presets.json")
    .expect("DB not found.");

    let ifs = presets.find_ifs_by("Barnsley fern").expect("Couldn't find Barnsley");

    println!("{:?}", ifs);

    let now = Instant::now();
    let sample = ChaosGame::run_chaos_game(ifs, 100_000_000);
    //let density = DensityEstimator2D::from(sample.as_slice()).kde_const(1024, 1024, 1.0);
    //let density = DensityEstimator2D::from(sample.as_slice()).histogram(8024, 8024);

    let density = DensityEstimator2D::from(sample.as_slice()).kde_adapt(8024, 8024);

    let mut integral = 0.0;

    for x in 0..density.width() {
        for y in 0..density.height() {
            integral += density[Index2D::from(x, y)];
        }
    }

    println!("PDE Integral = {}, error = {}%", integral, (1.0-integral).abs() * 100.0);

    let img = RgbRenderer::img_bw(&density);

    let elapsed = now.elapsed();
    img.save("Barnsley fern.png").unwrap();
    println!("Generated {}x{} image in {} (s)", density.width(), density.height(), elapsed.as_secs_f32());
}
