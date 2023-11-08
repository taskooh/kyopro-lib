// succint indexable dictionaries

use std::cmp::min;

/// able to access and get rank in O(1), select in O(log n)
/// space complexity is O(n)
/// This struct is immutable
trait SuccintBitVectorTrait {
    const LEVEL_LARGE: usize;
    const LEVEL_SMALL: usize;
    fn new_from(v: Vec<bool>) -> Self;
    fn access(&self, i: usize) -> bool;
    fn rank_all(&self, b: bool) -> usize;
    fn rank(&self, b: bool, i: usize) -> usize;
    fn select(&self, b: bool, i: usize) -> Option<usize>;
}

struct SuccintBitVector {
    original_vec: Vec<bool>,
    large_vec: Vec<usize>,
    small_vec: Vec<u16>,
    rank_all: [usize; 2],
}

impl SuccintBitVector {
    fn get_large(&self, b: bool, large_index: usize) -> usize {
        if b {
            self.large_vec[large_index]
        } else {
            large_index * Self::LEVEL_LARGE as usize - self.large_vec[large_index]
        }
    }

    fn get_small(&self, b: bool, small_index: usize) -> u16 {
        if b {
            self.small_vec[small_index]
        } else {
            (small_index % (Self::LEVEL_LARGE / Self::LEVEL_SMALL)) as u16
                * Self::LEVEL_SMALL as u16
                - self.small_vec[small_index]
        }
    }
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
        let mut rank_all = [0; 2];
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
                rank_all[1] += 1;
            } else {
                rank_all[0] += 1;
            }
        }
        if cur_small != 0 {
            small_vec.push(cur_small);
        }
        SuccintBitVector {
            original_vec: v,
            large_vec,
            small_vec,
            rank_all,
        }
    }

    /// O(1)
    /// access i-th element (0-indexed)
    fn access(&self, i: usize) -> bool {
        self.original_vec[i]
    }

    fn rank_all(&self, b: bool) -> usize {
        self.rank_all[b as usize]
    }

    /// O(1)
    /// get the number of b in [0, i)
    fn rank(&self, b: bool, i: usize) -> usize {
        let mut rank1 = 0;
        rank1 += self.large_vec[i / Self::LEVEL_LARGE];
        rank1 += self.small_vec[i / Self::LEVEL_SMALL] as usize;
        rank1 += self.original_vec
            [(i / Self::LEVEL_SMALL as usize) * Self::LEVEL_SMALL as usize..i]
            .iter()
            .filter(|&&x| x)
            .count();
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
    fn select(&self, b: bool, i: usize) -> Option<usize> {
        let rank = i + 1;
        if rank > self.rank_all[b as usize] {
            return None;
        }
        let mut large_left = 0;
        let mut large_right = self.large_vec.len();
        while large_left < large_right {
            let mid = (large_left + large_right) / 2;
            let rank_large_mid = self.get_large(b, mid);
            if rank_large_mid < rank {
                large_left = mid + 1;
            } else {
                large_right = mid;
            }
        }
        let large_index = large_right - 1;
        let rank_subtract = self.get_large(b, large_index);
        let rank_remain = (rank - rank_subtract) as u16;
        let mut small_left = large_index * Self::LEVEL_LARGE / Self::LEVEL_SMALL;
        let mut small_right = min(
            small_left + Self::LEVEL_LARGE / Self::LEVEL_SMALL,
            self.small_vec.len(),
        );
        while small_left < small_right {
            let mid = (small_left + small_right) / 2;
            let rank_small_mid = self.get_small(b, mid);
            if rank_small_mid < rank_remain as u16 {
                small_left = mid + 1;
            } else {
                small_right = mid;
            }
        }
        let small_index = small_right - 1;
        let rank_subtract = self.get_small(b, small_index);
        let mut rank_remain = rank_remain - rank_subtract;
        let mut index = small_index * Self::LEVEL_SMALL;
        while rank_remain > 0 {
            if self.original_vec[index] == b {
                rank_remain -= 1;
            }
            index += 1;
        }
        return Some(index - 1);
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
        let len = 10000;
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
