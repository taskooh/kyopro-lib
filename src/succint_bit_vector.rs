// succint indexable dictionaries

/// able to access and get rank in O(1), select in O(log n)
/// space complexity is O(n)
/// This struct is immutable
trait SuccintBitVectorTrait {
    const LEVEL_LARGE: usize;
    const LEVEL_SMALL: usize;
    fn new_from(v: Vec<bool>) -> Self;
    fn access(&self, i: usize) -> bool;
    fn rank(&self, b: bool, i: usize) -> usize;
    fn select(&self, b: bool, i: usize) -> Option<usize>;
}

struct SuccintBitVector {
    original_vec: Vec<bool>,
    large_vec: Vec<usize>,
    small_vec: Vec<u16>,
}

impl SuccintBitVectorTrait for SuccintBitVector {
    // n = 1 << 32;
    // 32 * 32;
    const LEVEL_LARGE: usize = 1024;
    // 32 / 2
    const LEVEL_SMALL: usize = 16;

    fn new_from(v: Vec<bool>) -> Self {
        let size = v.len();
        // log(len)^2
        let mut cur_large = 0;
        let mut cur_small = 0;
        let mut large_vec = vec![];
        let mut small_vec = vec![];
        for i in 0..size {
            if i % Self::LEVEL_LARGE as usize == 0 {
                large_vec.push(cur_large);
                cur_small = 0;
            }
            if i % Self::LEVEL_SMALL as usize == 0 {
                small_vec.push(cur_small);
            }
            if v[i] {
                cur_large += 1;
                cur_small += 1;
            }
        }
        if cur_small != 0 {
            small_vec.push(cur_small);
        }
        SuccintBitVector {
            original_vec: v,
            large_vec,
            small_vec,
        }
    }

    /// O(1)
    /// access i-th element (0-indexed)
    fn access(&self, i: usize) -> bool {
        self.original_vec[i]
    }

    /// O(1)
    /// get the number of b in [0, i)
    fn rank(&self, b: bool, i: usize) -> usize {
        let mut rank1 = 0;
        rank1 += self.large_vec[i / Self::LEVEL_LARGE];
        rank1 += self.small_vec[i / Self::LEVEL_SMALL] as usize;
        rank1 += self.original_vec[(i / Self::LEVEL_SMALL as usize) * Self::LEVEL_SMALL as usize .. i].iter().filter(|&&x| x).count();
        if b {
            rank1
        } else {
            i - self.rank(true, i)
        }
    }

    /// O(log n)
    /// get the index of i-th b (0-indexed)
    /// if there is no b, return None
    /// if i is out of range, return None
    /// TODO: optimize
    // fn select(&self, b: bool, i: usize) -> Option<usize> {
    //     let mut left = 0;
    //     let mut right = self.original_vec.len();
    //     while left < right{
    //         let mid = (left + right) / 2;
    //         if self.rank(b, mid + 1) < i + 1 {
    //             left = mid + 1;
    //         } else {
    //             right = mid;
    //         }
    //     }
    //     if right == self.original_vec.len() {
    //         None
    //     } else {
    //         Some(left)
    //     }
    // }
    fn select(&self, b: bool, i: usize) -> Option<usize> {
        let rank = i + 1;
        let mut large_left = 0;
        let mut large_right = self.large_vec.len();
        while large_left < large_right {
            let large_mid = (large_left + large_right) / 2;
            if self.large_vec[large_mid] <= rank {
                large_left = large_mid + 1;
            } else {
                large_right = large_mid;
            }
        }
        let large_index = large_right - 1;
        let small_rank_left = (rank - self.large_vec[large_index]) as u16;
        let mut small_left = large_index * Self::LEVEL_LARGE / Self::LEVEL_SMALL;
        let mut small_right = small_left + Self::LEVEL_LARGE / Self::LEVEL_SMALL;
        while small_left < small_right {
            let small_mid = (small_left + small_right) / 2;
            if self.small_vec[small_mid] <= small_rank_left as u16 {
                small_left = small_mid + 1;
            } else {
                small_right = small_mid;
            }
        }
        let small_index = small_right - 1;
        // Todo
        return None;
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_succint_index_dict() {
        let mut rng = rand::thread_rng();
        let mut v = vec![];
        let len = 100000;
        for _ in 0..len {
            let x = rng.gen::<bool>();
            v.push(x);
        }
        let dict = SuccintBitVector::new_from(v.clone());
        for i in 0..len {
            assert_eq!(dict.access(i), v[i]);
        }
        {
            let mut rank1 = 0;
            for i in 0..len {
                if v[i] {
                    rank1 += 1;
                }
                assert_eq!(dict.rank(true, i + 1), rank1);
                assert_eq!(dict.rank(false, i + 1), i + 1 - rank1);
            }
        }
        let mut true_count = 0;
        let mut false_count = 0;
        for i in 0..len {
            if v[i] {
                assert_eq!(dict.select(true, true_count), Some(i));
                true_count += 1;
            } else {
                assert_eq!(dict.select(false, false_count), Some(i));
                false_count += 1;
            }
        }
    }
}
