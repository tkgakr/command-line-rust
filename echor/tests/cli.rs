use assert_cmd::Command;
use predicates::prelude::*;

// 引数なしでプログラムを実行し、使い方が標準エラーに出力されることを検証する
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

// 引数を与えて実行し、正常に終了することを検証する
#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}
