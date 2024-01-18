fn main() {
    let path = "ui/appwindow.slint";
    let config = slint_build::CompilerConfiguration::default().with_style(String::from("material"));
    slint_build::compile_with_config(path, config).unwrap()
}
