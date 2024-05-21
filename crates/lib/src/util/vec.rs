use cargo_snippet::snippet;
pub use std::cmp;

// not usable in macro export
// pub trait Reorder {
//     fn reorder(&mut self, w: &Vec<usize>);
// }

// impl<T: Copy> Reorder for Vec<T> {
//     fn reorder(&mut self, w: &Vec<usize>) {
//         let mut nv = Vec::new();
//         for i in w {
//             nv.push(self[*i]);
//         }
//         *self = nv;
//     }
// }

/// Sorts the given vectors by the given function.
/// Requires `use lib::util::vec::*;`
///
/// # Examples
///
/// ```rust
/// use lib::srt;
/// use lib::util::vec::*;
/// let mut x = vec![1, 3, 2];
/// let mut y = vec![3, 2, 1];
/// srt!(|i, j| x[i] < x[j], y, x);
/// assert_eq!(x, [1, 2, 3]);
/// assert_eq!(y, [3, 1, 2]);
/// ```
#[snippet("srt")]
#[macro_export]
macro_rules! srt {
    ($f:expr, $head:expr, $($tail:expr),*) => {
        fn reorder<T: Copy>(slf: &mut Vec<T>, w: &Vec<usize>) {
            let mut nv = Vec::new();
            for i in w {
                nv.push(slf[*i]);
            }
            *slf = nv;
        }

        let mut w: Vec<usize> = (0..$head.len()).collect();
        w.sort_unstable_by(|&i, &j| if $f(i, j) { cmp::Ordering::Less } else { cmp::Ordering::Greater });
        reorder(&mut $head, &w);
        $(
            reorder(&mut $tail, &w);
        )*
    };
}

#[snippet("smaller")]
pub trait Comparator {
    fn smaller(&self) -> impl Fn(usize, usize) -> bool;
    fn greater(&self) -> impl Fn(usize, usize) -> bool;
}
impl<T: Ord> Comparator for Vec<T> {
    fn smaller(&self) -> impl Fn(usize, usize) -> bool {
        move |i, j| self[i] < self[j]
    }
    fn greater(&self) -> impl Fn(usize, usize) -> bool {
        move |i, j| self[i] > self[j]
    }
}

pub trait Decrement {
    fn decrement(&mut self);
}

impl<T: Copy + std::ops::SubAssign<T> + From<u8>> Decrement for Vec<T> {
    fn decrement(&mut self) {
        for i in 0..self.len() {
            self[i] -= 1.into();
        }
    }
}

impl Decrement for usize {
    fn decrement(&mut self) {
        *self -= 1;
    }
}

impl Decrement for i64 {
    fn decrement(&mut self) {
        *self -= 1;
    }
}

impl Decrement for i32 {
    fn decrement(&mut self) {
        *self -= 1;
    }
}

/// Decrements the given number or vector.
/// Requires `use lib::util::vec::*;`
///
/// # Examples
///
/// ```rust
/// use lib::i0;
/// use lib::util::vec::*;
/// let mut l = 1;
/// let mut r = 3;
/// let mut a = vec![1, 3, 2];
/// i0!(l, a);
/// assert_eq!(l, 0);
/// assert_eq!(a, [0, 2, 1]);
/// ```
#[macro_export]
macro_rules! i0 {
    ($($x:expr),*) => {
        $(
            $x.decrement();
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srt_0() {
        let mut x = vec![1, 3, 2];
        let mut y = vec!["c", "b", "aa"];
        srt!(|i, j| x[i] < x[j], y, x);
        assert_eq!(x, [1, 2, 3]);
        assert_eq!(y, ["c", "aa", "b"]);
    }

    #[test]
    fn test_srt_1() {
        let mut x = vec![1, 3, 2];
        let mut y = vec!["c", "b", "aa"];
        srt!(x.smaller(), y, x);
        assert_eq!(x, [1, 2, 3]);
        assert_eq!(y, ["c", "aa", "b"]);
    }

    #[test]
    fn test_i0() {
        let mut n = 3;
        let mut a = vec![1, 3, 2];
        i0!(n, a);
        assert_eq!(n, 2);
        assert_eq!(a, [0, 2, 1]);
    }
}
