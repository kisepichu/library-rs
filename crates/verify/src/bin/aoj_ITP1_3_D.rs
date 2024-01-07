// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D

use lib::math::divisors::divisors;
use lib::util::io::{read_line, Scanner};

fn main() {
    let r = read_line();
    let mut s = Scanner::new(&r);
    let a: i64 = s.next();
    let b: i64 = s.next();
    let c: i64 = s.next();

    // let mut res = 0;
    // for i in a..=b {
    //     if c % i == 0 {
    //         res += 1;
    //     }
    // }
    // println!("{}", res);

    let d = divisors(c);
    let res = d.iter().filter(|&&x| a <= x && x <= b).count();
    println!("{}", res);
}
