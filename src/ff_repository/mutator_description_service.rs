use std::collections::HashMap;
use serde::Deserialize;
use crate::ff_repository::json_helper::JsonHelper;
use crate::ff_repository::repository_error::RepositoryError;

#[derive(Deserialize, Default, Clone, Debug)]
pub(crate) struct MutatorDescription {
    pub enum_id: u8,
    pub screen_name: String,
    pub is_rand: bool,
    pub requires_affine: bool,
    pub param_order: Option<Vec<String>>,
    pub params: Option<Vec<MutatorParam>>
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct MutatorParam {
    pub name: String,
    pub lower_bound: f32,
    pub upper_bound: f32,
    pub default_value: f32
}

pub(crate) struct MutatorDescriptionService {
    mutator_desc: Vec<MutatorDescription>
}

impl AsRef<Vec<MutatorDescription>>  for MutatorDescriptionService {
    fn as_ref(&self) -> &Vec<MutatorDescription> {
        &self.mutator_desc
    }
}

impl MutatorDescriptionService {
    pub(crate) fn load(db_path: &str) -> Result<Self, RepositoryError> {
        match JsonHelper::read_db(db_path) {
            Ok(json) => {
                JsonHelper::parse_data::<HashMap<String, MutatorDescription>>(&json).map(|parsed_data|  {
                    Self { mutator_desc: Self::transform(&parsed_data) }
                })
            }
            Err(error) => { Err(error) }
        }
    }

    fn transform(mutator_desc: &HashMap<String, MutatorDescription>) -> Vec<MutatorDescription> {
        let mut result = vec![MutatorDescription::default(); mutator_desc.len()];

        mutator_desc.values().for_each(|desc| {
            let index = desc.enum_id as usize - 1;
            result[index] = desc.clone();
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::ff_repository::mutator_description_service::MutatorDescription;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "cylinder": {
                "enum_id": 29,
                "screen_name": "cylinder",
                "is_rand": false,
                "requires_affine": false
              },
          "perspective": {
            "enum_id": 30,
            "screen_name": "perspective",
            "is_rand": false,
            "requires_affine": false,
            "param_order": [
              "p1_angle",
              "p2_dist"
            ],
            "params": [
              {
                "name": "p1_angle",
                "lower_bound": 0,
                "upper_bound": 3.1415926535897,
                "default_value": 1.46
              },
              {
                "name": "p2_dist",
                "lower_bound": -1,
                "upper_bound": 1,
                "default_value":1
              }
            ]
          }
        }
        "#;

        type ParseResult = serde_json::Result<HashMap<String, MutatorDescription>>;
        let ifs = match serde_json::from_str(&json) as ParseResult {
            Ok(vec_affine_ifs) => Ok(vec_affine_ifs),
            Err(error) => Err(error),
        };

        eprintln!("{:?}", ifs);
    }
}