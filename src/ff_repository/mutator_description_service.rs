use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct MutatorDescription {
    pub enum_id: u8,
    pub screen_name: String,
    pub is_rand: bool,
    pub requires_affine: bool,
    pub param_order: Option<Vec<String>>,
    pub params: Option<Vec<MutatorParam>>
}

#[derive(Deserialize, Debug)]
pub(crate) struct MutatorParam {
    pub name: String,
    pub lower_bound: f32,
    pub upper_bound: f32,
    pub default_value: f32
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

/*impl<'de> Deserialize<'de> for AffineTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[rustfmt::skip]
        #[allow(non_camel_case_types)]
        #[derive(Deserialize)]
        enum Fields { a, b, c, d, e, f, p }

        struct AffineTransformVisitor;
        impl<'de> Visitor<'de> for AffineTransformVisitor {
            type Value = AffineTransform;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                return formatter.write_str("expecting AffineTransform");
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
            {
                let mut a: Option<f32> = None;
                let mut b: Option<f32> = None;
                let mut c: Option<f32> = None;
                let mut d: Option<f32> = None;
                let mut e: Option<f32> = None;
                let mut f: Option<f32> = None;
                let mut p: Option<f32> = None;

                // jfc. Visitor sees map, and expects its keys to be of the type
                // inferred from match. To parse this type it invokes Visitor for FieldsType (which is auto derived).
                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::a => a = map.next_value()?,
                        Fields::b => b = map.next_value()?,
                        Fields::c => c = map.next_value()?,
                        Fields::d => d = map.next_value()?,
                        Fields::e => e = map.next_value()?,
                        Fields::f => f = map.next_value()?,
                        Fields::p => p = map.next_value()?,
                    }
                }

                let a = a.ok_or_else(|| serde::de::Error::missing_field("a"))?;
                let b = b.ok_or_else(|| serde::de::Error::missing_field("b"))?;
                let c = c.ok_or_else(|| serde::de::Error::missing_field("c"))?;
                let d = d.ok_or_else(|| serde::de::Error::missing_field("d"))?;
                let e = e.ok_or_else(|| serde::de::Error::missing_field("e"))?;
                let f = f.ok_or_else(|| serde::de::Error::missing_field("f"))?;
                let p = p.ok_or_else(|| serde::de::Error::missing_field("p"))?;

                return Ok(AffineTransform {
                    mat: AffineMat::from(arr2(&[[a, b, c], [d, e, f]])),
                    p: p,
                });
            }
        }

        return deserializer.deserialize_map(AffineTransformVisitor);
    }
}*/