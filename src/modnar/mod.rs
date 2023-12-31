use std::ops::RangeInclusive;
use crate::util::Len;

pub(crate) mod rnd_vec;

pub struct Modnar {
    seed: u64,
    generator: fn(&mut Self) -> u64
}

impl Default for Modnar {
    fn default() -> Self {
        Self {
            seed: 0,
            generator: dummy,
        }
    }
}

impl Modnar {
    pub(crate) fn new_lsfr(seed: u64) -> Self {
        Self { seed: seed, generator: lsfr_ }
    }

    pub(crate) fn new_rng() -> Self {
        Self { seed: random_seed_(), generator: rng_}
    }

    pub(crate) fn gen(&mut self, range: RangeInclusive<u64>) -> u64 {
        let val = self.gen_f64();
        range.start().wrapping_add((range.len() as f64 * val).round() as u64)
    }

    pub(crate) fn gen_f32(&mut self) -> f32 {
        self.gen_f64() as f32
    }

    fn gen_f64(&mut self) -> f64 {
        f64::from_bits((self.generator)(self) >> 12 | 0x3ff0000000000000) - 1.0
    }
}

fn dummy(r: &mut Modnar) -> u64 {
    panic!("Using unimplemented modnar!")
}

fn lsfr_(r: &mut Modnar) -> u64 {
    r.seed = (r.seed >> 8) ^ BYTE_FEEDBACK[(r.seed & 0xff) as usize];
    r.seed as u64
}

fn rng_(rng: &mut Modnar) -> u64 {
    // from rust frand https://github.com/engusmaze/frand/blob/main/src/gen/mod.rs
    let mut value = rng.seed.wrapping_add(12964901029718341801);
    rng.seed = value;
    value = value.wrapping_mul(149988720821803190 ^ value);
    value ^ value >> 32
}

fn random_seed_() -> u64 {
    fn mix2_64(x: u64, y: u64) -> u64 {
        x.wrapping_add(y) ^ y << 10
    }
    fn hash_time() -> u64 {
        let duration = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Failed to get current time");
        mix2_64(duration.as_secs(), duration.subsec_nanos() as u64)
    }

    hash_time()
}

/// LSFR byte feedback
// - TODO: probably will crash with overflown stack (on Windows main thread stack is about 1MB, other threads it can be set).
const BYTE_FEEDBACK: [u64; 256] = [
    0x0000000000000000, 0xc70000000000000b, 0x8e0000000000000d, 0x4900000000000006,
    0x1c00000000000001, 0xdb0000000000000a, 0x920000000000000c, 0x5500000000000007,
    0x3800000000000002, 0xff00000000000009, 0xb60000000000000f, 0x7100000000000004,
    0x2400000000000003, 0xe300000000000008, 0xaa0000000000000e, 0x6d00000000000005,
    0x7000000000000004, 0xb70000000000000f, 0xfe00000000000009, 0x3900000000000002,
    0x6c00000000000005, 0xab0000000000000e, 0xe200000000000008, 0x2500000000000003,
    0x4800000000000006, 0x8f0000000000000d, 0xc60000000000000b, 0x0100000000000000,
    0x5400000000000007, 0x930000000000000c, 0xda0000000000000a, 0x1d00000000000001,
    0xe000000000000008, 0x2700000000000003, 0x6e00000000000005, 0xa90000000000000e,
    0xfc00000000000009, 0x3b00000000000002, 0x7200000000000004, 0xb50000000000000f,
    0xd80000000000000a, 0x1f00000000000001, 0x5600000000000007, 0x910000000000000c,
    0xc40000000000000b, 0x0300000000000000, 0x4a00000000000006, 0x8d0000000000000d,
    0x900000000000000c, 0x5700000000000007, 0x1e00000000000001, 0xd90000000000000a,
    0x8c0000000000000d, 0x4b00000000000006, 0x0200000000000000, 0xc50000000000000b,
    0xa80000000000000e, 0x6f00000000000005, 0x2600000000000003, 0xe100000000000008,
    0xb40000000000000f, 0x7300000000000004, 0x3a00000000000002, 0xfd00000000000009,
    0xc00000000000000b, 0x0700000000000000, 0x4e00000000000006, 0x890000000000000d,
    0xdc0000000000000a, 0x1b00000000000001, 0x5200000000000007, 0x950000000000000c,
    0xf800000000000009, 0x3f00000000000002, 0x7600000000000004, 0xb10000000000000f,
    0xe400000000000008, 0x2300000000000003, 0x6a00000000000005, 0xad0000000000000e,
    0xb00000000000000f, 0x7700000000000004, 0x3e00000000000002, 0xf900000000000009,
    0xac0000000000000e, 0x6b00000000000005, 0x2200000000000003, 0xe500000000000008,
    0x880000000000000d, 0x4f00000000000006, 0x0600000000000000, 0xc10000000000000b,
    0x940000000000000c, 0x5300000000000007, 0x1a00000000000001, 0xdd0000000000000a,
    0x2000000000000003, 0xe700000000000008, 0xae0000000000000e, 0x6900000000000005,
    0x3c00000000000002, 0xfb00000000000009, 0xb20000000000000f, 0x7500000000000004,
    0x1800000000000001, 0xdf0000000000000a, 0x960000000000000c, 0x5100000000000007,
    0x0400000000000000, 0xc30000000000000b, 0x8a0000000000000d, 0x4d00000000000006,
    0x5000000000000007, 0x970000000000000c, 0xde0000000000000a, 0x1900000000000001,
    0x4c00000000000006, 0x8b0000000000000d, 0xc20000000000000b, 0x0500000000000000,
    0x6800000000000005, 0xaf0000000000000e, 0xe600000000000008, 0x2100000000000003,
    0x7400000000000004, 0xb30000000000000f, 0xfa00000000000009, 0x3d00000000000002,
    0x800000000000000d, 0x4700000000000006, 0x0e00000000000000, 0xc90000000000000b,
    0x9c0000000000000c, 0x5b00000000000007, 0x1200000000000001, 0xd50000000000000a,
    0xb80000000000000f, 0x7f00000000000004, 0x3600000000000002, 0xf100000000000009,
    0xa40000000000000e, 0x6300000000000005, 0x2a00000000000003, 0xed00000000000008,
    0xf000000000000009, 0x3700000000000002, 0x7e00000000000004, 0xb90000000000000f,
    0xec00000000000008, 0x2b00000000000003, 0x6200000000000005, 0xa50000000000000e,
    0xc80000000000000b, 0x0f00000000000000, 0x4600000000000006, 0x810000000000000d,
    0xd40000000000000a, 0x1300000000000001, 0x5a00000000000007, 0x9d0000000000000c,
    0x6000000000000005, 0xa70000000000000e, 0xee00000000000008, 0x2900000000000003,
    0x7c00000000000004, 0xbb0000000000000f, 0xf200000000000009, 0x3500000000000002,
    0x5800000000000007, 0x9f0000000000000c, 0xd60000000000000a, 0x1100000000000001,
    0x4400000000000006, 0x830000000000000d, 0xca0000000000000b, 0x0d00000000000000,
    0x1000000000000001, 0xd70000000000000a, 0x9e0000000000000c, 0x5900000000000007,
    0x0c00000000000000, 0xcb0000000000000b, 0x820000000000000d, 0x4500000000000006,
    0x2800000000000003, 0xef00000000000008, 0xa60000000000000e, 0x6100000000000005,
    0x3400000000000002, 0xf300000000000009, 0xba0000000000000f, 0x7d00000000000004,
    0x4000000000000006, 0x870000000000000d, 0xce0000000000000b, 0x0900000000000000,
    0x5c00000000000007, 0x9b0000000000000c, 0xd20000000000000a, 0x1500000000000001,
    0x7800000000000004, 0xbf0000000000000f, 0xf600000000000009, 0x3100000000000002,
    0x6400000000000005, 0xa30000000000000e, 0xea00000000000008, 0x2d00000000000003,
    0x3000000000000002, 0xf700000000000009, 0xbe0000000000000f, 0x7900000000000004,
    0x2c00000000000003, 0xeb00000000000008, 0xa20000000000000e, 0x6500000000000005,
    0x0800000000000000, 0xcf0000000000000b, 0x860000000000000d, 0x4100000000000006,
    0x1400000000000001, 0xd30000000000000a, 0x9a0000000000000c, 0x5d00000000000007,
    0xa00000000000000e, 0x6700000000000005, 0x2e00000000000003, 0xe900000000000008,
    0xbc0000000000000f, 0x7b00000000000004, 0x3200000000000002, 0xf500000000000009,
    0x980000000000000c, 0x5f00000000000007, 0x1600000000000001, 0xd10000000000000a,
    0x840000000000000d, 0x4300000000000006, 0x0a00000000000000, 0xcd0000000000000b,
    0xd00000000000000a, 0x1700000000000001, 0x5e00000000000007, 0x990000000000000c,
    0xcc0000000000000b, 0x0b00000000000000, 0x4200000000000006, 0x850000000000000d,
    0xe800000000000008, 0x2f00000000000003, 0x6600000000000005, 0xa10000000000000e,
    0xf400000000000009, 0x3300000000000002, 0x7a00000000000004, 0xbd0000000000000f,
];

#[cfg(test)]
mod tests {
    use super::{Modnar, random_seed_};
    use std::collections::HashMap;

    #[test]
    fn test_lsfr() {
        for _ in 0..100 {
            let mut map = HashMap::<u64, u32>::new();
            let mut lsfr = Modnar::new_lsfr(random_seed_());

            for _ in 0..100 {
                let val = lsfr.gen(0..=(1 << 50));
                if let Some(count) = map.get(&val) {
                    map.insert(val, *count + 1);
                } else {
                    map.insert(val, 1);
                }
            }

            // - todo: check here(curvy shit).
            // LSFR gives uniformly distributed vals 0..2^64 - 1.
            // Making it narrower, means same vals will end up in the same bucket.
            // Bucket width is the 2^64 / (input range).
            // 1..10 -> 1..5
            // 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10
            //   1 |   2   |   3   |   4   |   5   |
            // Now why do we need lsfr?
        }
    }

    #[test]
    fn test_rng() {
        let mut rng1 = Modnar::new_rng();
        let mut rng2 = Modnar::new_rng();

        let mut vec1 = Vec::<u64>::new();
        let mut vec2 = Vec::<u64>::new();

        for _ in 0..100 {
            vec1.push(rng1.gen(10..=100));
            vec2.push(rng2.gen(10..=100));
        }

        let mut counter_eq = 0;
        for it in vec1.iter().zip(vec2.iter()) {
            let (ai, bi) = it;
            assert!(*ai >= 10 && *ai <= 100);
            assert!(*bi >= 10 && *bi <= 100);

            counter_eq += if *ai == *bi {
                1
            } else {
                0
            }
        }

        assert!(counter_eq < 10);
    }

    #[test]
    fn test_rng_f32() {
        let mut rng1 = Modnar::new_rng();
        let mut rng2 = Modnar::new_rng();

        let mut vec1 = Vec::<f32>::new();
        let mut vec2 = Vec::<f32>::new();

        for _ in 0..100 {
            vec1.push(rng1.gen_f32());
            vec2.push(rng2.gen_f32());
        }

        let mut counter_eq = 0;
        for it in vec1.iter().zip(vec2.iter()) {
            let (ai, bi) = it;
            assert!(*ai >= 0.0 && *ai <= 1.0);
            assert!(*bi >= 0.0 && *bi <= 1.0);

            counter_eq += if *ai == *bi {
                1
            } else {
                0
            }
        }

        assert!(counter_eq < 10);
    }

}