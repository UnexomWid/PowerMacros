extern crate winresource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        println!(
            "cargo:warning=Target Environment: {}",
            std::env::var("CARGO_CFG_TARGET_ENV").unwrap()
        );
        let res = winresource::WindowsResource::new();
        res.compile().unwrap();
    }
}
