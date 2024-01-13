# library-rs

[![verify](https://github.com/tqkoh/library-rs/actions/workflows/verify.yml/badge.svg)](https://github.com/tqkoh/library-rs/actions/workflows/verify.yml) [![pages-build-deployment](https://github.com/tqkoh/library-rs/actions/workflows/pages/pages-build-deployment/badge.svg?branch=gh-pages)](https://tqk.blue/library-rs/lib/)
  
競技プログラミング用ライブラリ [doc](https://tqk.blue/library-rs/rustdoc/lib/)  

全然追加してないけど C++ 版もある: [tqkoh/library](https://github.com/tqkoh/library)

### 準備

- Rust とかインストール

## スニペット出力

コマンドは全部リポジトリルートで実行

```sh
$ rustup component add rustfmt
$ cargo install cargo-snippet --features="binaries"
$ cargo snippet crates/lib -t vscode
```

## その他

### テスト

```sh
$ cargo test --all-targets
```

### ローカルでドキュメントを見る

```sh
$ cargo doc --no-deps
$ explorer.exe .\\target\\doc\\lib\\index.html
```

### ローカルで verify (実際の問題を使いテスト)

```sh
$ pip3 install online-judge-verify-helper
$ oj-verify run
```

### lib ファイルの追加

```sh
$ bash add_lib_file.sh math divisors
```

e.g. [crates/lib/src/math/divisors.rs](crates/lib/src/math/divisors.rs)



### verify 問題の追加

```sh
$ bash
$ source add_verification_problem.sh itp1_3_d https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_3_D
```

して crates/verify/src/{problem}.rs の fn {problem} 内に解法を書く

- crates/verify/src/bin/{problem}.rs から呼んでる


### todo

- [ ] verify log の見た目を整える
- [ ] add_*.sh をいい感じにしたい(けど Rust で cli にしたりテンプレートエンジンとか使うほどか.?)
