use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use crate::ds::affine_mat::AffineMat;

#[derive(Debug, Clone)]
pub(crate) struct IfsTransform {
    pub mat: AffineMat,
    pub p: f32,
}

impl<'de> Deserialize<'de> for IfsTransform {
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
            type Value = IfsTransform;

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

                return Ok(IfsTransform {
                    mat: AffineMat::from(a, b, c, d, e, f),
                    p,
                });
            }
        }

        return deserializer.deserialize_map(AffineTransformVisitor);
    }
}