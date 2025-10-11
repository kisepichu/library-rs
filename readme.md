# library-rs

[![verify](https://github.com/kisepichu/library-rs/actions/workflows/verify.yml/badge.svg)](https://github.com/tqkoh/library-rs/actions/workflows/verify.yml) [![GitHub Pages](https://img.shields.io/static/v1?label=GitHub+Pages&message=+&color=brightgreen&logo=github)](https://kisepichu.github.io/library-rs/lib/)

競技プログラミング用ライブラリ [doc](https://kisepichu.github.io/library-rs/lib/)

全然追加してないけど C++ 版もある: [tqkoh/library](https://github.com/tqkoh/library)

### 準備

- Rust とかインストール

## スニペット出力

コマンドは全部リポジトリルートで実行

```sh
$ rustup component add rustfmt
$ cargo install cargo-snippet --features="binaries"
$ cargo snippet crates/lib -t vscode | clip.exe
```

`| clip` して vscode で `Ctrl + Shift + P` で `Snippets: Open User Snippets` で `rust.json` を開いて貼り付けるか、 `> /mnt/c/Users/tqk/AppData/Roaming/Code/User/snippets/rust.json` するとか

## その他

### テスト

```sh
$ cargo test --all-targets
```

### ローカルでドキュメントを見る

```sh
$ cargo doc --no-deps && explorer.exe .\\target\\doc\\lib\\index.html
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
