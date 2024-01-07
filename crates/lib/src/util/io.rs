/// Scanner for competitive programming
///
/// # Examples
///
/// ```
/// use io::Scanner;
///
/// let mut s = Scanner::new("abc 334\n121"); // Scanner::new(read_line());
/// assert_eq!(s.next::<String>(), "abc");
/// assert_eq!(s.next::<i64>(), 334);
/// assert_eq!(s.next::<i64>(), 121);
/// ```
///
use std;
use std::str::FromStr;

pub struct Scanner<'a> {
    iter: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            iter: s.split_whitespace(),
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        let s = self.iter.next().unwrap();
        if let Ok(v) = s.parse::<T>() {
            v
        } else {
            panic!("Parse error")
        }
    }

    pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
        let n: usize = self.next();
        self.next_vec(n)
    }

    pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

pub fn read_string() -> String {
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
}

pub fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::util::io::Scanner;

    #[test]
    fn test_scanner() {
        let s = "abc 334\n121";
        let mut sc = Scanner::new(s);
        assert_eq!(sc.next::<String>(), "abc");
        assert_eq!(sc.next::<i64>(), 334);
        assert_eq!(sc.next::<i64>(), 121);
    }
}
