use crate::ds::aff_ifs::AffIfs;
use crate::ds::ifs_transform::IfsTransform;
use crate::ff_repository::json_helper::JsonHelper;
use crate::ff_repository::repository_error::RepositoryError;

pub(crate) struct PresetsRepository {
    pub(crate) affine_presets: Vec<AffIfs>,
    pub(crate) flatted: Vec<IfsTransform>
}

impl PresetsRepository {
    pub(crate) fn load(db_path: &str) -> Result<Self, RepositoryError> {
        // - TODO: how to forward error btw maps?
        match JsonHelper::read_db(db_path) {
            Ok(json) => {
                JsonHelper::parse_data::<Vec<AffIfs>>(&json)
                    .map(|parse_result| {
                        let mut self_ = Self {
                            affine_presets: parse_result,
                            flatted: Vec::<IfsTransform>::new()
                        };
                        self_.post_process();
                        self_
                    })
            },
            Err(error) => { Err(error) }
        }
    }

    pub(crate) fn find_ifs_by(&self, name: &str) -> Option<&AffIfs> {
        self.affine_presets.iter().find(|ifs| ifs.name == name)
    }

    fn post_process(&mut self) {
        for affine_ifs in &mut self.affine_presets {
            affine_ifs.prepare_preset_for_chaos_game();

            // - WARNING: it's important that we add those to the flatted array only after transforms in the ifs
            // are sorted. Need to introduce some sort of lock on this order.
            self.flatted.append(&mut affine_ifs.transforms.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ds::aff_ifs::AffIfs;

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

        type ParseResult = serde_json::Result<Vec<AffIfs>>;
        let ifs = match serde_json::from_str(&json) as ParseResult {
            Ok(vec_affine_ifs) => Ok(vec_affine_ifs),
            Err(error) => Err(error),
        };

        eprintln!("{:?}", ifs);
    }
}
