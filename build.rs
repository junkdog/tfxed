fn main() {
    let web_backend = std::env::var("CARGO_FEATURE_WEB_BACKEND").is_ok();
    let crossterm_backend = std::env::var("CARGO_FEATURE_CROSSTERM_BACKEND").is_ok();

    if web_backend && crossterm_backend {
        panic!("Features 'web' and 'crossterm-backend' cannot be enabled simultaneously");
    }
}