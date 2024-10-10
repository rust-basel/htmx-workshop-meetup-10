use tower_http::services::ServeDir;

pub fn using_serve_dir() -> ServeDir {
    ServeDir::new("assets")
}
