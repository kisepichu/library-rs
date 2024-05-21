/// b ? 1 : 0.
///
/// # Examples
///
/// ```rust
/// use lib::util::other::one;
/// let mut count = 0;
/// count += one("some" < "condition");
/// ```
#[inline]
pub fn one(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

/// n != 0.
///
/// # Examples
///
/// ```rust
/// use lib::util::other::nz;
/// let mut q = 3;
/// while nz(q) { q -= 1;}
#[inline]
pub fn nz<T: PartialEq + From<u8>>(n: T) -> bool {
    n != 0.into()
}

/// Update mn = min(mn, n), and return if mn was updated.
///
/// # Examples
///
/// ```rust
/// use lib::util::other::chmin;
/// let mut mn = 998244353;
/// if chmin(&mut mn, 3) {}
/// ```
#[inline]
pub fn chmin<T: PartialOrd>(mn: &mut T, n: T) -> bool {
    if *mn > n {
        *mn = n;
        true
    } else {
        false
    }
}

/// Update mn = max(mn, n), and return if mn was updated.
///
/// # Examples
///
/// ```rust
/// use lib::util::other::chmax;
/// let mut mn = -998244353;
/// if chmax(&mut mn, 3) {}
/// ```
#[inline]
pub fn chmax<T: PartialOrd>(mn: &mut T, n: T) -> bool {
    if *mn < n {
        *mn = n;
        true
    } else {
        false
    }
}
