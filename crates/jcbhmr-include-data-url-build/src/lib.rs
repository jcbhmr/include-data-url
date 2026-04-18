use base64::prelude::*;
use cargo_toml::Manifest;
use regex::{Captures, Regex};
use std::{
    env,
    error::Error,
    path::{self, PathBuf},
};

fn replace_all_err<T: AsRef<str>, E>(
    regex: &Regex,
    haystack: &str,
    mut rep: impl FnMut(&Captures<'_>) -> Result<T, E>,
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for cap in regex.captures_iter(haystack) {
        let m = cap.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        let replacement = rep(&cap)?;
        new.push_str(replacement.as_ref());
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

pub fn metabuild() {
    fn inner() -> Result<(), Box<dyn Error>> {
        let cargo_manifest_path =
            env::var_os("CARGO_MANIFEST_PATH").ok_or_else(|| "CARGO_MANIFEST_PATH not set")?;
        let cargo_manifest_path = PathBuf::from(cargo_manifest_path);
        let manifest = Manifest::from_path(&cargo_manifest_path)?;
        if let Some(product) = manifest.lib.as_ref() {
            let lib_path = product
                .path
                .as_deref()
                .ok_or_else(|| format!("lib.path should exist"))?;
            let lib_path = path::absolute(lib_path)?;
            let lib_text = fs_err::read_to_string(&lib_path)?;
            // 1: `/* jcbhmr_include_data_url_build::include_data_url!("`
            // 2: `path/to/file`
            // 3: `", "`
            // 4: `mime/type`
            // 5: `") */ "`
            // 6: `data:mime/type;base64,encoded_data`
            // 7: `"`
            let regex = Regex::new(
                r#"(/\*\*?\s*jcbhmr_include_data_url_build::include_data_url\!\s*\(\s*")([^"]*)("\s*,\s*")([^"]*)("\s*\)\s*\*/\s*")([^"]*)(")"#,
            ).expect("regex should compile");
            let new_lib_text =
                replace_all_err(&regex, &lib_text, |cap| -> Result<_, Box<dyn Error>> {
                    let path = cap[2].to_owned();
                    let mime = cap[4].to_owned();
                    let data = fs_err::read(&path)?;
                    let encoded_data = BASE64_STANDARD.encode(&data);
                    let data_url = format!("data:{};base64,{}", &mime, &encoded_data);
                    Ok(format!(
                        "{}{}{}{}{}{}{}",
                        &cap[1], &cap[2], &cap[3], &cap[4], &cap[5], data_url, &cap[7]
                    ))
                })?;
            if new_lib_text != lib_text {
                fs_err::write(&lib_path, &new_lib_text)?;
            }
        }
        Ok(())
    }
    inner().unwrap();
}
