fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let _ = embed_resource::compile("wincmd.rc", &[""]);
    }
}
