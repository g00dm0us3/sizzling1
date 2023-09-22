use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::fs::read_to_string;

use crate::ff_repository::affine_transform::AffineIfs;

#[derive(Debug)]
pub(crate) enum PresetServiceError {
    FileNotFound,

    // wtf is trait object
    JSONDecoding,
}

impl Display for PresetServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl Error for PresetServiceError {}

pub(crate) struct PresetService {
    pub(crate) affine_presets: Vec<AffineIfs>,
}

impl PresetService {
    pub(crate) fn load(db_path: &str) -> Result<Self, PresetServiceError> {
        let json: String = Self::read_db(db_path)?;
        let mut self_ = Self::parse_data(json)?;
        self_.post_process();
        return Ok(self_);
    }

    pub(crate) fn find_ifs_by(&self, name: &str) -> Option<&AffineIfs> {
        self.affine_presets.iter().find(|ifs| ifs.name == name)
    }

    fn read_db(db_path: &str) -> Result<String, PresetServiceError> {
        match read_to_string(db_path) {
            Ok(string_data) => return Ok(string_data),
            Err(_) => return Err(PresetServiceError::FileNotFound),
        };
    }

    fn parse_data(json: String) -> Result<Self, PresetServiceError> {
        type ParseResult = serde_json::Result<Vec<AffineIfs>>;

        match serde_json::from_str(&json) as ParseResult {
            Ok(vec_affine_ifs) => {
                return Ok(Self {
                    affine_presets: vec_affine_ifs,
                })
            }
            Err(_) => return Err(PresetServiceError::JSONDecoding),
        };
    }

    fn post_process(&mut self) {
        for affine_ifs in &mut self.affine_presets {
            AffineIfsPreprocessor::sort_ifs(affine_ifs);
            AffineIfsPreprocessor::set_cumulative_probs(affine_ifs);
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
