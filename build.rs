fn main() {
    if version_check::is_min_version("1.46.0").unwrap_or(false) {
        println!("cargo:rustc-cfg=compiler_has_important_bugfix");
    }
}
