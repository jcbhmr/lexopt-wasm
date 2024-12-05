use std::{
    env::{args, var},
    error::Error,
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    assert!(Command::new("cargo")
        .args(&["component", "build",])
        .status()?
        .success());
    assert!(Command::new("jco")
        .args(&[
            "transpile",
            "target/wasm32-wasip1/debug/lexopt_wasm.wasm",
            "-o",
            "target/jco"
        ])
        .status()?
        .success());
    std::fs::write(
        "target/jco/package.json",
        r#"{
            "type": "module",
            "dependencies": {
                "@bytecodealliance/preview2-shim": "*"
            }
        }"#,
    )?;
    assert!(Command::new("npm")
        .current_dir("target/jco")
        .args(&["install"])
        .status()?
        .success());
    assert!(Command::new("node")
        .args(&[
            "--input-type=module",
            "--eval",
            r#"
                import * as lexopt_wasm from "./target/jco/lexopt_wasm.js";
                console.log(lexopt_wasm);
            "#,
        ])
        .status()?
        .success());
    Ok(())
}
