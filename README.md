# command-line-rust

『[コマンドラインRust（Rust 練習帳）](https://www.oreilly.co.jp/books/9784814400584/)』（オライリー・ジャパン）の写経・学習用リポジトリです。

書籍を読み進めながら、各章で作成するコマンドラインプログラムを Rust で実装していきます。

## 参考

- 書籍: [Rustの練習帳 ―コマンドラインツールの作成を通してRustを学ぶ](https://www.oreilly.co.jp/books/9784814400584/)
- 原著サンプルコード: [kyclark/command-line-rust](https://github.com/kyclark/command-line-rust)

## 環境

- Rust（edition 2024）
- Cargo

## ディレクトリ構成

各章ごとに独立した Cargo パッケージを作成します。

## ビルドと実行

各パッケージのディレクトリで以下を実行します。

```sh
# 例: hello パッケージを実行
cd hello
cargo run
```

ビルドのみ行う場合:

```sh
cargo build
```

テストを実行する場合:

```sh
cargo test
```
