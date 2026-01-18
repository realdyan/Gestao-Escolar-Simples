// build.rs
fn main() {
    slint_build::compile_with_config(
        "ui/main.slint",
        slint_build::CompilerConfiguration::new()
            .with_include_paths(vec!["ui".into()])
    ).unwrap();
}