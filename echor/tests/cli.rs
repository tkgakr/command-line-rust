use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// 引数なしでプログラムを実行し、使い方が標準エラーに出力されることを検証する
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

// 引数を与えて実行し、期待した結果で正常終了することを検証する
#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}
