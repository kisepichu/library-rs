use std::fmt::Display;

/// Unzip a vector of tuples into multiple vectors.
/// Requires `use paste;`
///
/// # Examples
/// ```rust
/// use lib::unzip;
/// use lib::util::io::*;
///
/// let ab = vec![(1, 2), (3, 4), (5, 6)];
/// unzip!((a, b), ab);
/// a; // [1, 3, 5]
/// b; // [2, 4, 6]
/// ```
#[macro_export]
macro_rules! unzip {
    (($($a:ident),*), $ab:expr) => {
        $(let mut $a = Vec::new();)*
        let it = $ab.into_iter();
        for e in it {
            paste::item!{
                let ($([<elem_ $a>]),*) = e;
            }
            paste::item!{
                $(
                    $a.push([<elem_ $a>]);
                )*
            }
        }
    };
}

/// b ? "Yes" : "No"
#[allow(non_snake_case)]
pub fn Yes(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}

/// b ? "yes" : "no"
pub fn yes(b: bool) {
    println!("{}", if b { "yes" } else { "no" });
}

/// b ? "Possible" : "Impossible"
#[allow(non_snake_case)]
pub fn Possible(b: bool) {
    println!("{}", if b { "possible" } else { "impossible" });
}

/// b ? "possible" : "impossible"
pub fn possible(b: bool) {
    println!("{}", if b { "possible" } else { "impossible" });
}

pub trait OutFormatter {
    fn out_format(&self) -> String;
}

impl<T: Display> OutFormatter for Vec<T> {
    fn out_format(&self) -> String {
        self.iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl OutFormatter for usize {
    fn out_format(&self) -> String {
        self.to_string()
    }
}

impl OutFormatter for f64 {
    fn out_format(&self) -> String {
        format!("{:.10}", self)
    }
}

impl OutFormatter for String {
    fn out_format(&self) -> String {
        self.clone()
    }
}

impl OutFormatter for char {
    fn out_format(&self) -> String {
        self.to_string()
    }
}

impl OutFormatter for &str {
    fn out_format(&self) -> String {
        self.to_string()
    }
}

/// Print given values separated by spaces to stdout.
/// Requires `use lib::util::io::*`
///
/// # Examples
/// ```rust
/// use lib::out;
/// use lib::util::io::*;
/// let v = vec![9, 9, 8];
/// let s = "string";
/// let n = 998;
/// out!(v, s, n); // "9 9 8 string 998"
/// ```
#[macro_export]
macro_rules! out {
    ($($arg:expr),*) => {
        println!("{}", vec![$($arg.out_format()),*].join(" "));
    };
}

/// Print given values separated by spaces to stderr.
/// Requires `use lib::util::io::*`
///
/// # Examples
/// ```rust
/// use lib::deb;
/// use lib::util::io::*;
/// let v = vec![9, 9, 8];
/// let s = "string";
/// let n = 998;
/// deb!(v, s, n); // "9 9 8 string 998"
/// ```
#[macro_export]
macro_rules! deb {
    ($($arg:expr),*) => {
        eprintln!("{}", vec![$($arg.out_format()),*].join(" "));
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_unzip_two() {
        let ab = vec![
            (1, "a".to_string()),
            (3, "b".to_string()),
            (5, "c".to_string()),
        ];
        unzip!((a, b), ab);
        assert_eq!(a, vec![1, 3, 5]);
        assert_eq!(b, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }
    #[test]
    fn test_unzip_three() {
        let ab = vec![(1, 2, "a"), (3, 4, "b"), (5, 6, "c")];
        unzip!((a, b, c), ab);
        assert_eq!(a, vec![1, 3, 5]);
        assert_eq!(b, vec![2, 4, 6]);
        assert_eq!(c, vec!["a", "b", "c"]);
    }
}
