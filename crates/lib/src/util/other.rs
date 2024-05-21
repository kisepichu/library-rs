use cargo_snippet::snippet;

#[snippet]
pub fn one(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}
