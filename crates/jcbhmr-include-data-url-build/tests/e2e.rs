use dedent::dedent;
use derive_more::{From, Into};
use std::{collections::HashMap, error::Error, fs, io, path, process::Command};

#[test]
fn include_data_url_build() -> Result<(), Box<dyn Error>> {
    let root = path::absolute("./tests/e2e/include_data_url_build")?;
    _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root)?;
    fs::write(root.join(".gitignore"), "*")?;
    fs::write(
        root.join("Cargo.toml"),
        dedent!(
            r#"
                [workspace]

                [package]
                name = "include_data_url_build"
                version = "0.0.0"
                edition = "2024"

                [build-dependencies]
                jcbhmr-include-data-url-build = { path = "../../.." }
            "#
        ),
    )?;
    fs::write(
        root.join("build.rs"),
        dedent!(
            r#"
                fn main() {
                    jcbhmr_include_data_url_build::metabuild();
                }
            "#
        ),
    )?;
    fs::create_dir_all(root.join("src"))?;
    fs::write(
        root.join("src/lib.rs"),
        dedent!(
            r#"
                #![doc(
                    html_favicon_url = /* jcbhmr_include_data_url_build::include_data_url!("src/favicon.ico", "image/x-icon") */ "",
                    html_logo_url = /* jcbhmr_include_data_url_build::include_data_url!("src/logo.png", "image/png") */ "",
                )]
            "#
        ),
    )?;
    fs::write(root.join("src/favicon.ico"), b"fake icon data")?;
    fs::write(root.join("src/logo.png"), b"fake logo data")?;

    let output = Command::new("cargo")
        .arg("doc")
        .current_dir(&root)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "{}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        )
        .into());
    }

    let html = fs::read_to_string(
        root.join("target/doc/include_data_url_build/index.html")
    )?;
    assert!(html.contains("data:image/x-icon;"));
    assert!(html.contains("data:image/png;"));

    Ok(())
}
