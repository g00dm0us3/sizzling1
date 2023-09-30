#[derive(Debug, Clone)]
pub(crate) struct AffineMat {
    pub(crate) a: f32,
    pub(crate) b: f32,
    pub(crate) c: f32,
    pub(crate) d: f32,
    pub(crate) e: f32,
    pub(crate) f: f32
}

impl AffineMat {
    pub(crate) fn from(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
        }
    }

    pub(crate) fn det(&self) -> f32 {
        /*
        - TODO: formalize as a type layout
        a b e
        c d f
        */
        self.a*self.d - self.b*self.c
    }
}