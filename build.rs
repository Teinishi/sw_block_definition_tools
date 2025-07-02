fn main() {
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        embed_resource::compile("app.rc");
    }
}
