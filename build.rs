// build.rs
fn main() {

    // qt -> The Qt style
    // native -> This is an alias to one of the other styles depending on the platform
    // cosmic -> The Cosmic variants emulate the style used by Cosmic Desktop
    // cupertino -> The Cupertino variants emulate the style used by macOS.
    // material -> These variants are part of the Material style
    // fluent ->    These variants belong to the Fluent style

    // Tell Cargo to recompile the project if the .slint files are changed
    slint_build::compile_with_config(
        "ui/app.slint",
        slint_build::CompilerConfiguration::new().with_style("fluent".into())
    ).unwrap();
}
