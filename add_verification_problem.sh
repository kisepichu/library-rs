# verify 問題を追加する
# $1: 問題名
# $2: url
# ex. source add_verification_problem.sh aoj_itp1_3_d https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D
# ミスったら undo_add_verification_problem を実行

if [ $# -ne 2 ]; then
  echo "invalid arguments"
  echo "ex. source add_verification_problem.sh aoj_itp1_3_d https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D"
  return
fi

# crates/verify/Cargo.toml に
# [[bin]]
# name = "aoj_itp1_3_d"
# path = "src/bin/aoj_itp1_3_d.rs"
# doc = false
#
#
# を追加。虚無の main.rs が doc に入らないように
cargo_toml_body="\n[[bin]]\nname = \"$1\"\npath = \"src/bin/$1.rs\"\ndoc = false\n"
echo -e $cargo_toml_body >>crates/verify/Cargo.toml

# crates/verify/src/lib.rs に
# pub mod aoj_itp1_3_d;
# を追加
lib_rs_body="pub mod $1;\n"
echo -e "$lib_rs_body" >>crates/verify/src/lib.rs

# crates/verify/src/aoj_itp1_3_d.rs
echo "//! solution for <$2>

pub fn $1() {
    // todo: solve $2
}
" >>crates/verify/src/$1.rs

# crates/verify/src/bin/aoj_itp1_3_d.rs
echo "// verification-helper: PROBLEM $2

use verify::$1;

fn main() {
    $1::$1();
}
" >>crates/verify/src/bin/$1.rs

# ミスったとき消す用

name=$1
url=$2

delete_prob_in_cargo_toml() {
  if [ $# -lt 2 ]; then
    return
  fi

  python3 -c "
name = '$1'
path = '$2'

result = [] # 消さない行を append していく
d = False

with open(path, 'r') as f:
    lines = f.readlines()
    for i, line in enumerate(lines):
        if line.strip() == 'name = \"' + name + '\"':
            d = True
            result.pop()
        elif d and line == '\n':
            d = False
        elif not d:
            result.append(line)

with open(path, 'w') as f:
    f.writelines(result)
"
}

undo_add_verification_problem() {
  echo "remove added contents in crates/verify/Cargo.toml: "
  delete_prob_in_cargo_toml $name crates/verify/Cargo.toml

  echo "remove added contents in crates/verify/src/lib.rs: "
  lib_rs_escaped=$(echo -e "$lib_rs_body" | sed -e 's/\\/\\\\/g')
  sed -i -e "s/$lib_rs_escaped//g" crates/verify/src/lib.rs

  echo "remove crates/verify/src/bin/$name.rs: "
  cat crates/verify/src/bin/$name.rs
  rm crates/verify/src/bin/$name.rs

  echo "remove crates/verify/src/$name.rs: "
  cat crates/verify/src/$name.rs
  rm crates/verify/src/$name.rs

  undo_add_verification_problem() {
    echo "手動で直してね :abao_iyaaaa:"
  }
}
