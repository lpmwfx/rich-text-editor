fn main() {
    slintscanners::scan_project();
    rustscanners::scan_project();

    let config = slint_build::CompilerConfiguration::new()
        .with_include_paths(vec![std::path::PathBuf::from("../RustUI/ui")]);
    slint_build::compile_with_config("ui/main.slint", config)
        .expect("Slint compilation failed");
}
