use cargo_snippet::snippet;

use std::collections::HashSet;

/// Returns a HashSet containing the divisors of the given number.
///
/// # Examples
///
/// ```rust
/// use lib::math::divisors::divisors;
/// assert_eq!(divisors(6), [1, 2, 3, 6].iter().cloned().collect());
/// ```
#[snippet("divisors")]
#[snippet(prefix = "use std::collections::HashSet;")]
pub fn divisors(n: i64) -> HashSet<i64> {
    let mut res = HashSet::new();
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res.insert(i);
            if i * i != n {
                res.insert(n / i);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(1), [1].iter().cloned().collect());
        assert_eq!(divisors(2), [1, 2].iter().cloned().collect());
        assert_eq!(divisors(3), [1, 3].iter().cloned().collect());
        assert_eq!(divisors(4), [1, 2, 4].iter().cloned().collect());
    }
}
