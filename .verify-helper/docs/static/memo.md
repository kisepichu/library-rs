
### lib の追加

crates/lib/src/{category}/{a-de}.rs に書き、 crates/lib/src/{category}/mod.rs に pub mod {a-de}; を追加

[crates/lib/src/math/divisors.rs](crates/lib/src/math/divisors.rs)

### verify 問題の追加

```
source add_verification_problem.sh itp1_3_d https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D
```

[add_verification_problem.sh](add_verification_problem.sh)

して crates/verify/src/{problem}.rs の fn {problem} 内に書く

- crates/verify/src/bin/{problem}.rs から呼んでる


### gos

いろいろ

```
$ cargo test --all
$ cargo doc
$ explorer.exe . # ./target/doc/lib
$ oj-verify run
$ git pull --rebase # oj-verify がプッシュしてくるため
```
