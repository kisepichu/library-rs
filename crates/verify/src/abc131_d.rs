//! solution for <https://atcoder.jp/contests/abc131/tasks/abc131_d>

use lib::util::other::Yes;
use lib::util::vec::*;
use lib::*;
use proconio::input;

pub fn abc131_d() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    }
    // ab.sort_by_key(|&(_, b)| b);
    // let mut ok = true;
    // let mut c = 0;
    // for i in 0..n {
    //     ok &= c + ab[i].0 <= ab[i].1;
    //     c += ab[i].0;
    // }
    // Yes(ok);

    let (mut a, mut b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();

    srt!(b.smaller(), a, b);
    let mut ok = true;
    let mut c = 0;

    for i in 0..n {
        ok &= c + a[i] <= b[i];
        c += a[i];
    }
    Yes(ok);
}
