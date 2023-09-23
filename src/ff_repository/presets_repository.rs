use std::cmp::Ordering;
use crate::ff_repository::affine_transform::{AffineIfs, AffineTransform};
use crate::ff_repository::json_helper::JsonHelper;
use crate::ff_repository::repository_error::RepositoryError;

pub(crate) struct PresetsRepository {
    pub(crate) affine_presets: Vec<AffineIfs>,
    pub(crate) flatted: Vec<AffineTransform>
}

impl PresetsRepository {
    pub(crate) fn load(db_path: &str) -> Result<Self, RepositoryError> {
        // - TODO: how to forward error btw maps?
        match JsonHelper::read_db(db_path) {
            Ok(json) => {
                JsonHelper::parse_data::<Vec<AffineIfs>>(&json)
                    .map(|parse_result| {
                        let mut self_ = Self {
                            affine_presets: parse_result,
                            flatted: Vec::<AffineTransform>::new()
                        };
                        self_.post_process();
                        self_
                    })
            },
            Err(error) => { Err(error) }
        }
    }

    pub(crate) fn find_ifs_by(&self, name: &str) -> Option<&AffineIfs> {
        self.affine_presets.iter().find(|ifs| ifs.name == name)
    }

    fn post_process(&mut self) {
        for affine_ifs in &mut self.affine_presets {
            AffineIfsPreprocessor::sort_ifs(affine_ifs);
            AffineIfsPreprocessor::set_cumulative_probs(affine_ifs);

            // - WARNING: it's important that we add those to the flatted array only after transforms in the ifs
            // are sorted. Need to introduce some sort of lock on this order.
            self.flatted.append(&mut affine_ifs.transforms.clone());
        }
    }
}

// service that reads json knows awfully a lot about what it reads
struct AffineIfsPreprocessor;
impl AffineIfsPreprocessor {
    fn sort_ifs(ifs: &mut AffineIfs) {
        ifs.transforms.sort_by(|rhs, lhs| {
            return if rhs.p > lhs.p {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        });
    }

    fn set_cumulative_probs(ifs: &mut AffineIfs) {
        let mut cum_prob = 0.0;

        // - TODO: is there an idiom for this??
        let last_idx = ifs.transforms.len() - 1;
        for (idx, transform) in ifs.transforms.iter_mut().enumerate() {
            cum_prob += transform.p;
            transform.p = cum_prob;

            if idx == last_idx { transform.p = 1.0; }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ff_repository::affine_transform::AffineIfs;

    #[test]
    fn test_json_parse() {
        let json = r#"
        [
            {
                "name": "Serpinski gasket",
                "transforms": [
                    {  "a": 0.5, "b": 0, "c": 0, "d": 0, "e": 0.5, "f": 0, "p": 0.33 },
                    {  "a": 0.5, "b": 0, "c": 0.5, "d": 0, "e": 0.5, "f": 0, "p": 0.33 },
                    {  "a": 0.5, "b": 0, "c": 0, "d": 0, "e": 0.5, "f": 0.5, "p": 0.33 }
                ]
            },
            {
                "name": "Serpinski gasket copy",
                "transforms": [
                    {  "a": 0.5, "b": -0.35, "c": -0.5, "d": 0.35, "e": 0.35, "f": 0, "p": 0.33 },
                    {  "a": 0.35, "b": -0.35, "c": 0.5, "d": 0.35, "e": 0.5, "f": 0, "p": 0.33 },
                    {  "a": 0.35, "b": -0.35, "c": 0, "d": 0.35, "e": 0.35, "f": 0.5, "p": 0.33 }
                ]
            }
        ]
        "#;

        type ParseResult = serde_json::Result<Vec<AffineIfs>>;
        let ifs = match serde_json::from_str(&json) as ParseResult {
            Ok(vec_affine_ifs) => Ok(vec_affine_ifs),
            Err(error) => Err(error),
        };

        eprintln!("{:?}", ifs);
    }
}
