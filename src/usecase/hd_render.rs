use std::collections::HashSet;
use std::time::Instant;
use crate::alg::combinations::Combinations;
use crate::chaos_game::ChaosGame;
use crate::ds::aff_ifs::ChaosGamePreprocess;
use crate::ds::array_2d::Array2D;
use crate::ds::ifs_transform::IfsTransform;
use crate::ff_repository::mutator_description_service::MutatorDescriptionService;
use crate::ff_repository::presets_repository::PresetsRepository;
use crate::frac_render::RgbRenderer;
use crate::mutators::{MutatorConfig, Mutators};
use crate::statistics::grid_density::DensityEstimator2D;

// Use color-steal for color mapping.
pub(crate) struct HDRender;

impl HDRender {
    pub(crate) fn render(
        draw_sz: u8,
        p_rank: u64,
        m_rank: Option<u64>,
        presets_repo: &PresetsRepository,
        mutator_repo: Option<&MutatorDescriptionService>,
        iterations: RenderIter,
        use_kde: bool,
        img_height: usize,
        img_width: usize
    ) {
        let mut combinations = Combinations::new();
        let comb: HashSet<usize> = combinations
            .unrank(p_rank, presets_repo.flatted.len() as u8, draw_sz)
            .into_iter()
            .map(|e| (e - 1) as usize)
            .collect();

        let mut ifs: Vec<IfsTransform> = presets_repo
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

        ifs.prepare_for_chaos_game(true);
        let iter = iterations as u32;
        let instant = Instant::now();
        let mut chaos_game = ChaosGame::new();
        let samples = chaos_game
            .run_chaos_game(&ifs, Some(&[
                MutatorConfig::new(0.25, Mutators::Swirl),
                //MutatorConfig::new(0.25, Mutators::Bent),
                //MutatorConfig::new(0.25, Mutators::Julian { power: 5.0, dist: 0.31 }),
                //MutatorConfig::new(0.25, Mutators::RadianBlur { angle: 1.27, v36: -5.5 })
            ]), iter);
        let density: Array2D;

        if !use_kde {
            density = DensityEstimator2D::new(&samples).histogram(img_height, img_width);
        } else {
            density = DensityEstimator2D::new(&samples).kde_adapt(img_height, img_width);
        }

        println!("Compute density in {}", instant.elapsed().as_secs_f32());

        let render_method = if use_kde { "kde" } else { "hist" };
        let num_of_iter = format!("{iter:e}");

        let img = RgbRenderer::img_bw_simple(&density);
        // 97074 is the rank of that particular permutation.
        img.save(&format!("C:\\sizzling_hd_renders\\{draw_sz}.{p_rank}.4.3.{render_method}.{num_of_iter}.png")).unwrap();
    }
}

#[repr(u32)]
pub(crate) enum RenderIter {
    Small = 10_000_000,
    Large = 50_000_000,
    FuckingMassive = 100_000_000,
    Gargantuan = 1_000_000_000
}
