#![doc(
    html_favicon_url = "https://docs.rs/crate/jcbhmr-include-data-url/0.2.0/source/src/favicon.ico"
)]
#![doc(html_logo_url = "https://docs.rs/crate/jcbhmr-include-data-url/0.2.0/source/src/logo.png")]
//! This package provides one procedural macro: [`include_data_url!`].

use base64::prelude::*;
use derive_more::{From, Into};
use quote::quote;
use std::{env, fs, path};
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// A procedural macro that includes the contents of a file as a `data:` URL
/// string literal. Similar to [`include_str!`].
///
/// Useful for composing with
/// [`#[doc]`](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html)
/// where a standalone URL is expected but the author wants to reference a local
/// file.
///
/// **Signature:** `include_data_url!(path: literal_string, type: literal_string) -> literal_string`
///
/// - `path` is relative to the crate root, not the current file. The path must be within the crate as determined by `absolute(path).starts_with(cwd)`.
/// - `type` is the MIME type of the data that will be inlined. For example, `text/plain`, `image/jpeg`, etc.
///
/// # Example
///
/// Embed a crate-local image as a `data:` URL in a documentation comment.
///
/// <div><code>lib.rs</code></div>
///
/// ```
/// # use jcbhmr_include_data_url::include_data_url;
/// /// Look at this cat:
/// ///
/// /// ![](
/// #[doc = include_data_url!("./src/cat.jpg", "image/jpeg")]
/// /// )
/// struct Cat;
/// ```
///
/// <details><summary>Directory tree</summary>
///
/// ```txt
/// .
/// ├── src/
/// │   ├── cat.jpg
/// │   └── lib.rs
/// └── Cargo.toml
/// ```
///
/// </details>
///
/// <table>
/// <caption>Rendered</caption>
/// <tr><td>
///
/// ![](https://docs.rs/crate/jcbhmr-include-data-url/0.2.0/source/src/cat-screenshot.png)
///
/// </td></tr>
/// </table>
///
/// <b>ℹ️ Note:</b> This has [user-centric
/// performance](https://web.dev/articles/user-centric-performance-metrics#important-metrics)
/// implications if the inlined `data:` URL is large. You may wish to use a URL
/// like `https://docs.rs/crate/${package}/${version}/source/${path}` instead.
#[proc_macro]
pub fn include_data_url(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, From, Into)]
    struct Input {
        path: LitStr,
        type_: LitStr,
    }
    impl Parse for Input {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let path: LitStr = input.parse()?;
            let _: Token![,] = input.parse()?;
            let type_: LitStr = input.parse()?;
            Ok(Self { path, type_ })
        }
    }
    let input = parse_macro_input!(input as Input);
    let path = input.path.value();
    let type_ = input.type_.value();

    let current_dir = env::current_dir()
        .unwrap_or_else(|e| panic!("current directory should be accessible: {}", e));
    let path_absolute = path::absolute(&path)
        .unwrap_or_else(|e| panic!("{:?} should be absolute-izable: {}", path, e));
    if !path_absolute.starts_with(&current_dir) {
        panic!(
            "{:?} should be within the crate directory {:?}",
            path_absolute, current_dir
        );
    }

    let data = fs::read(&path_absolute)
        .unwrap_or_else(|e| panic!("{:?} should be readable: {}", path_absolute, e));
    let encoded = BASE64_STANDARD.encode(&data);
    let data_url = format!("data:{};base64,{}", type_, encoded);

    quote!(#data_url).into()
}
