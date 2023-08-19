
// a bunch of ways to select numbers at random LSFR, frand.
// somehow seeded.
// a bunch of different types of numbers. converts.

// - TODO: this probably goes into alg.
use std::ops::RangeInclusive;

struct Randomness {
    seed: u64,
    generator: fn(&mut Self) -> f32
}

// pretty pointless.
impl Randomness {
    pub(crate) fn lsfr() -> Self {
        return Self { seed: 7, generator: lsfr_ };
    }

    /*pub(crate) fn rng() -> Self {
        return Self { seed: 7 };
    }*/

    fn gen(&mut self, range: RangeInclusive<u64>) -> u64 {

    }
}

fn lsfr_(r: &mut Randomness) -> f32 {
    0
}

// seed maybe important (we want identical LSFR) / unimportant

