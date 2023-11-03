// lower_bound, upper_bound
use std::cmp::Ordering;

pub trait LowerBound<T> {
    fn lower_bound(&self, x: &T) -> usize;
}

pub trait UpperBound<T> {
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> LowerBound<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => low = mid + 1,
                Ordering::Equal | Ordering::Greater => high = mid,
            }
        }
        low
    }
}

impl<T: Ord> UpperBound<T> for [T] {
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => low = mid + 1,
                Ordering::Greater => high = mid,
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lower_bound() {
        use super::LowerBound;
        let v = vec![1, 2, 3, 3, 3, 5, 5, 6];
        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&1), 0);
        assert_eq!(v.lower_bound(&2), 1);
        assert_eq!(v.lower_bound(&3), 2);
        assert_eq!(v.lower_bound(&4), 5);
        assert_eq!(v.lower_bound(&5), 5);
        assert_eq!(v.lower_bound(&6), 7);
        assert_eq!(v.lower_bound(&7), 8);
    }

    #[test]
    fn test_upper_bound() {
        use super::UpperBound;
        let v = vec![1, 2, 3, 3, 3, 5, 5, 6];
        assert_eq!(v.upper_bound(&0), 0);
        assert_eq!(v.upper_bound(&1), 1);
        assert_eq!(v.upper_bound(&2), 2);
        assert_eq!(v.upper_bound(&3), 5);
        assert_eq!(v.upper_bound(&4), 5);
        assert_eq!(v.upper_bound(&5), 7);
        assert_eq!(v.upper_bound(&6), 8);
        assert_eq!(v.upper_bound(&7), 8);
    }
}
