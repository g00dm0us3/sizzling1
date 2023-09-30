use std::cmp::Ordering;
use serde::Deserialize;
use crate::ds::ifs_transform::IfsTransform;

#[derive(Deserialize, Debug)]
pub(crate) struct AffIfs {
    pub(crate) name: String,
    pub(crate) transforms: Vec<IfsTransform>,
}

pub(crate) trait ChaosGamePreprocess {
    fn prepare_for_chaos_game(&mut self, needs_reweigh: bool);
}

impl ChaosGamePreprocess for Vec<IfsTransform> {
    fn prepare_for_chaos_game(&mut self, needs_reweigh: bool) {
        fn sort_ifs(tfms: &mut Vec<IfsTransform>) {
            tfms.sort_by(|rhs, lhs| {
                return if rhs.p > lhs.p {
                    Ordering::Greater
                } else {
                    Ordering::Less
                };
            });
        }

        fn set_cumulative_probs(tfms: &mut Vec<IfsTransform>) {
            let mut cum_prob = 0.0;

            // - TODO: is there an idiom for this??
            let last_idx = tfms.len() - 1;
            for (idx, transform) in tfms.iter_mut().enumerate() {
                cum_prob += transform.p;
                transform.p = cum_prob;

                if idx == last_idx { transform.p = 1.0; }
            }
        }

        if !needs_reweigh {
            sort_ifs(self);
            set_cumulative_probs(self);
        } else {
            let total_det: f32 = self
                .iter()
                .map(|e| { e.mat.det().abs() })
                .sum();

            self.iter_mut().for_each(|t| {
                t.p = t.mat.det().abs()/total_det;
            });
        }
    }
}

impl AffIfs {
    pub(crate) fn prepare_preset_for_chaos_game(&mut self) {
        self.transforms.prepare_for_chaos_game(false);
    }
}