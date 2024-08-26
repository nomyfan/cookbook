fn main() {
    pkg_config::Config::new().probe("zlib").unwrap();
}
