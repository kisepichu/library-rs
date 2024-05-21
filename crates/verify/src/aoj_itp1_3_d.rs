//! solution for <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D>

use lib::math::divisors::divisors;
use lib::out;
use lib::util::io::OutFormatter;
use proconio::input;

pub fn aoj_itp1_3_d() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let d = divisors(c);
    let res = d.iter().filter(|&&x| a <= x && x <= b).count();
    out!(res);
}
