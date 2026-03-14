fn main() {
    slintscanners::scan_project();
    rustscanners::scan_project();

    slint_build::compile("ui/main.slint").expect("Slint compilation failed");
}
