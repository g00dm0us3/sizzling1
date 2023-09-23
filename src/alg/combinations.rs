use num_bigint::BigUint;
use num_traits::One;
use std::vec;

pub(crate) struct Combinations {
    cache: Vec<u64>
}

impl Combinations {
    const CACHE_SIZE: usize = u16::MAX as usize;

    pub(crate) fn new() -> Self { Self { cache: vec![0; Self::CACHE_SIZE] } }

    // - TODO: get rid of mut.
    pub(crate) fn combinations(&mut self, n: u8, k: u8) -> u64 {
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

    /// Returns a combination, using its lexicographical rank.
    /// # Arguments
    /// * `rank` - doi, rank.
    /// * `pool_sz` - how many element  are in the pool.
    /// * `draw_sz` - how many are we drawing.
    /// 
    /// # Discussion
    /// `pool_sz` and `draw_sz` are essentially the coordinates of the number of $C^{n}_{k}$
    /// Indicies of elements in pool start w. 1.
    pub(crate) fn unrank(&mut self, rank: u64, pool_sz: u8, draw_sz: u8) -> Vec<u8> {
        let mut c: Vec<u8> = Vec::new();
        let mut r = rank;
        let mut j: u8 = 0;

        for s in 1..=draw_sz {
            let mut cs = j + 1;

            if pool_sz < cs {
                return Vec::new();
            }

            while r > self.combinations(pool_sz - cs, draw_sz - s) {
                if pool_sz < cs {
                    return Vec::new();
                }

                r -= self.combinations(pool_sz - cs, draw_sz - s);
                cs += 1;
            }

            c.push(cs);
            j = cs;

        }

        return c;
    }

    /// Smallest rank is 1.
    pub(crate) fn rank(&mut self, combination: &Vec<u8>, pool_sz: u8, draw_sz: u8) -> u64 {
        let k = combination.len() as u8;
        let n = pool_sz;

        // there is a total of this many combinations
        let mut result = self.combinations(pool_sz,draw_sz);

        // - TODO: don't clone.
        let mut combination = combination.clone();
        combination.sort();

        for i in 0..combination.len() {
            result -= self.combinations(n - combination[i], k - (i as u8));
        }

        result
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

    #[test]
    fn test_unrank() {
        let mut combinations = Combinations::new();
        let mut combo = combinations.unrank(4, 4, 3);

        assert_eq!(combo, vec![2,3,4]);

        // good nuff
        combo = combinations.unrank(8, 5, 3);
        assert_eq!(combo, vec![2,3,5]);
    }

    #[test]
    fn test_rank() {
        let mut combinations = Combinations::new();
        let rank = combinations.rank(&vec![2,3,5], 5, 3);

        assert_eq!(rank, 8);

        let rank = combinations.rank(&vec![2,3,4], 4, 3);
        assert_eq!(rank, 4);

        let rank = combinations.rank(&vec![27], 48, 1);
        eprintln!("{:}", rank);
    }

}