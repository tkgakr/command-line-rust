fn main() {
    // get_args関数が Ok(config) を返した場合、and_then をつかって run に congif を渡す
    if let Err(e) = catr::get_args().and_then(catr::run) {
        //  get_args か run が Err を返したら標準エラーに出力
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
