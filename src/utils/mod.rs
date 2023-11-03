// frequently used modules for competitive programming
pub use std::cmp::{max, min};
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
pub use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};
pub use std::iter::{FromIterator, IntoIterator};
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
pub use std::str::FromStr;
pub use std::sync::Mutex;
pub use std::vec::Vec;
mod bound;
pub use bound::{LowerBound, UpperBound};

// frequently used macros for competitive programming
#[allow(unused_macros)]
macro_rules! chmax {
    ($a: expr, $b: expr) => {
        $a = std::cmp::max($a, $b);
    };
}
#[allow(unused_macros)]
macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = std::cmp::min($a, $b);
    };
}
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        {
            use std::io::Write;
            let stderr = std::io::stderr();
            let mut stderr = std::io::BufWriter::new(stderr.lock());
            writeln!(stderr, "{}", format_args!($($a),*)).unwrap();
        }
    };
}
