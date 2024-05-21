# lib ファイルを追加する
# $1: category
# $2: name
# ex. source add_lib_file.sh math gcd

if [ $# -ne 2 ]; then
  echo "Invalid arguments"
  echo "Usage: source add_lib_file.sh [category] [name]"
  echo "ex. source add_lib_file.sh math gcd"
  return
fi

category=$1
name=$2

# crates/lib/src/{category}/{name}.rs を作成
mkdir -p crates/lib/src/$category
if [ -e crates/lib/src/$category/$name.rs ]; then
  echo "File already exists"
else
  echo -e "use cargo_snippet::snippet;\n" >>crates/lib/src/$category/$name.rs
  echo -e "pub mod $name;\n" >>crates/lib/src/$category.rs
fi

code crates/lib/src/$category/$name.rs
