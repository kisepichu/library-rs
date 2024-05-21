use cargo_snippet::snippet;

#[snippet]
pub fn one(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

#[snippet]
#[allow(non_snake_case)]
pub fn Yes(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}

#[snippet]
pub fn yes(b: bool) {
    println!("{}", if b { "yes" } else { "no" });
}
