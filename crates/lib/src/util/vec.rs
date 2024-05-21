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
}
