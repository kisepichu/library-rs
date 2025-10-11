use crate::util::other::nz;

/// Greatest common divisor of a, b > 0.
///
/// # Examples
///
/// ```rust
/// use lib::math::gcd::gcd;
/// assert_eq!(gcd(9, 12), 3);
/// ```
#[inline]
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while nz(b) {
        let c = b;
        b = a % b;
        a = c;
    }
    a
}

/// Least common multiple of a, b > 0.
///
/// # Examples
///
/// ```rust
/// use lib::math::gcd::lcm;
/// assert_eq!(lcm(9, 12), 36);
/// ```
#[inline]
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(9, 12), 3);
        assert_eq!(gcd(12, 9), 3);
        assert_eq!(gcd(1, 9), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(9, 12), 36);
        assert_eq!(lcm(12, 9), 36);
        assert_eq!(lcm(1, 9), 9);
    }
}
