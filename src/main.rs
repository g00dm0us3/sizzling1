mod alg;
mod chaos_game;
mod ff_repository;
mod statistics;
mod util;
mod ds;
mod frac_render;
mod mutators;
mod modnar;
mod usecase;

use std::time::Instant;

use ff_repository::service::PresetService;
use statistics::grid_density::DensityEstimator2D;

use chaos_game::ChaosGame;

use crate::frac_render::RgbRenderer;
use crate::mutators::{MutatorConfig, Mutators};
use std::env;
use std::process::exit;

// grids, which are too small don't yield detailed results (all samples endup in the same bins)
// - TODO: quad-tree to grid?
// - TODO: supersampling should probably be a thing.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Path to IFS presets not set!");
        exit(-2);
    }
    let ifs_presets_json_path = &args[1];
    // - TODO: erase before commit.
    let presets = PresetService::load(ifs_presets_json_path)
        .expect("DB not found.");
    let ifs = presets.find_ifs_by("Barnsley fern").expect("Couldn't find Barnsley");
    let mut chaos_game = ChaosGame::new();

    let now = Instant::now();
    let sample = chaos_game.run_chaos_game(
        ifs,
        Some(&[MutatorConfig::new(1.0, Mutators::Swirl)]),
        100_000
    );

    let density = DensityEstimator2D::from(sample.as_slice()).histogram(256, 256);

    /*let mut integral = 0.0;

    for x in 0..density.width() {
        for y in 0..density.height() {
            integral += density[Index2D::from(x, y)];
        }
    }

    println!("PDE Integral = {}, error = {}%", integral, (1.0-integral).abs() * 100.0);
    */

    let img = RgbRenderer::img_bw_simple(&density);

    let elapsed = now.elapsed();

    img.save("Barnsley fern.png").unwrap();

    println!("Generated {}x{} image in {} (s)", density.width(), density.height(), elapsed.as_secs_f32());
}
