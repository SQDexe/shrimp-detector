use {
    slint_build::{
        compile_with_config,
        CompilerConfiguration
        },
    winresource::WindowsResource,
    std::env::var_os,
    };

fn main() {
    if var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("./assets/icon.ico")
            .compile()
            .expect("Icon build failed");
        }

    let config = CompilerConfiguration::new()
        .with_style(String::from("fluent-light"));

    compile_with_config("./ui/app.slint", config)
        .expect("Slint build failed");
    }