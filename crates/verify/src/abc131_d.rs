//! solution for <https://atcoder.jp/contests/abc131/tasks/abc131_d>

use lib::util::io::Yes;
use lib::util::vec::*;
use lib::*;
use paste;
use proconio::input;

pub fn abc131_d() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    }
    unzip!((a, b), ab);

    srt!(b.smaller(), a, b);
    let mut ok = true;
    let mut c = 0;

    for i in 0..n {
        ok &= c + a[i] <= b[i];
        c += a[i];
    }
    Yes(ok);
}
