use lib::math::divisors::divisors;
use lib::util::io::{read_line, Scanner};

pub fn aoj_itp1_3_d() {
    let r = read_line();
    let mut s = Scanner::new(&r);
    let a: i64 = s.next();
    let b: i64 = s.next();
    let c: i64 = s.next();

    let d = divisors(c);
    let res = d.iter().filter(|&&x| a <= x && x <= b).count();
    println!("{}", res);
}
