mod alg;
mod chaos_game;
mod ff_repository;
mod statistics;
mod util;
mod ds;
mod frac_render;
mod mutators;
mod usecase;
mod modnar;

use std::time::Instant;

use ff_repository::presets_repository::PresetsRepository;
use statistics::grid_density::DensityEstimator2D;

use chaos_game::ChaosGame;

use crate::frac_render::RgbRenderer;
use crate::mutators::{MutatorConfig, Mutators};
use std::env;
use std::process::exit;
use crate::ff_repository::mutator_description_service::{MutatorDescriptionService};
use crate::usecase::starship_enterprise::StarshipEnterprise;

// grids, which are too small don't yield detailed results (all samples endup in the same bins)
// - TODO: supersampling should probably be a thing.
fn main() {
    let args: Vec<String> = env::args().collect();

    const IFS_NAME: &str = "Serpinski carpet";
    if args.len() < 3 {
        eprintln!("Either path to IFS presets or to mutator desc not set!");
        exit(-2);
    }
    let ifs_presets_json_path = &args[1];
    // - TODO: erase before commit.
    let presets = PresetsRepository::load(ifs_presets_json_path)
        .expect("IFS presets not found!");

    let mut_desc_json_path = &args[2];
    let mut_desc = MutatorDescriptionService::load(mut_desc_json_path)
        .expect("Mutator descriptions not found!");

    let mut starship = StarshipEnterprise::new(&presets, &mut_desc);
    starship.roll_dice_presets("", 500);
    /*let ifs = presets.find_ifs_by(IFS_NAME).expect(&format!("Couldn't find {IFS_NAME}"));
    let mut chaos_game = ChaosGame::new();

    let now = Instant::now();
    let sample = chaos_game.run_chaos_game(
        &ifs,
        Some(&[
            MutatorConfig::new(0.25, Mutators::Disc),
            MutatorConfig::new(0.25, Mutators::Bent),
            MutatorConfig::new(0.25, Mutators::Julian { power: 5.0, dist: 0.31 }),
            MutatorConfig::new(0.25, Mutators::RadianBlur { angle: 1.27, v36: -5.5 })
        ]),
        200_000
    );

    let density = DensityEstimator2D::new(sample.as_slice()).histogram(256, 256);

    //let density = DensityEstimator2D::from(sample.as_slice()).kde_adapt(1024, 1024);
    /*let mut integral = 0.0;

    for x in 0..density.width() {
        for y in 0..density.height() {
            integral += density[Index2D::from(x, y)];
        }
    }

    println!("PDE Integral = {}, error = {}%", integral, (1.0-integral).abs() * 100.0);
    */

    let img = RgbRenderer::img_bw_simple(&density);

    //let img = RgbRenderer::img_bw_(&density);
    let elapsed = now.elapsed();

    img.save(&format!("{IFS_NAME}.png")).unwrap();

    println!("Generated {}x{} image in {} (s)", density.width(), density.height(), elapsed.as_secs_f32());
     */
}
