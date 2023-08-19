use num_bigint::BigUint;
use num_traits::One;

const CACHE_SIZE: usize = u16::MAX as usize;
struct Combinations {
    cache: [u64; CACHE_SIZE]
}

impl Combinations {
    fn new() -> Self { Self { cache: [0; CACHE_SIZE] } }

    // - TODO: get rid of mut.
    fn combinations(&mut self, n: u8, k: u8) -> u64 {
        if n == k { return 1; }
        if n < k { return 0; }

        if let Some(cached) = self.lookup(n, k) {
            return cached;
        }

        let combs: BigUint = n.factorial() / (k.factorial() * (n - k).factorial());
        let digits = combs.to_u64_digits();

        assert!(digits.len() == 1, "Arithmetic overflow!");

        let combs = digits[0];
        
        self.cache[Self::key(n, k)] = combs;

        combs
    }

    fn key(n: u8, k: u8) -> usize {
        let n = n as u16;
        let k = k as u16;
        let key = (n << 8) | k;

        key as usize
    }

    fn lookup(&self, n: u8, k: u8) -> Option<u64> {
        assert!(n != 0);
        let key = Self::key(n, k);
        if self.cache[key] == 0 { None } else { Some(self.cache[key]) }
    }
}

trait CombinatoricsOps {
    fn factorial(&self) -> BigUint;
} 

impl CombinatoricsOps for u8 {
    fn factorial(&self) -> BigUint {
        if *self == 0 { return One::one(); }
        let mut result: BigUint = One::one();

        // There is a more effective version of this.
        // But frankly, adding another cache would be a much better optimization.
        for idx in 1..=*self {
            result *= idx;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::alg::combinations::CombinatoricsOps;
    use super::Combinations;

    #[test]
    fn test_factorial_zero() {
        assert_eq!((0 as u8).factorial().to_u64_digits(),vec![1]);
    }

    #[test]
    fn test_factorial_small_int() {
        assert_eq!((2 as u8).factorial().to_u64_digits(), vec![2]);
        assert_eq!((3 as u8).factorial().to_u64_digits(), vec![6]);
        assert_eq!((4 as u8).factorial().to_u64_digits(), vec![24]);
        assert_eq!((5 as u8).factorial().to_u64_digits(), vec![120]);
    }

    #[test]
    fn test_compute_combinations_when_pool_equals_number_of_mutators() {
        let reference: Vec<u64> = vec![
            49,
            1176,
            18424,
            211876,
            1906884,
            13983816,
            85900584,
            450978066,
            2054455634,
            8217822536,
            29135916264,
            92263734836,
            262596783764,
            675248872536,
            1575580702584,
            3348108992991,
            6499270398159,
            11554258485616,
            18851684897584,
            28277527346376,
            39049918716424,
            49699896548176,
            58343356817424,
            63205303218876,
            63205303218876,
            58343356817424,
            49699896548176,
            39049918716424,
            28277527346376,
            18851684897584,
            11554258485616,
            6499270398159,
            3348108992991,
            1575580702584,
            675248872536,
            262596783764,
            92263734836,
            29135916264,
            8217822536,
            2054455634,
            450978066,
            85900584,
            13983816,
            1906884,
            211876,
            18424,
            1176,
            49,
            1
        ];

        let mut combinations = Combinations::new();
        let all_combinations: Vec<u64> = (1..=49).map(|elem| { combinations.combinations(49, elem) }).collect();

        assert_eq!(reference, all_combinations);
    }
}