//! This package provides one procedural macro: [`include_data_url!`].

use base64::prelude::*;
use derive_more::{From, Into};
use quote::quote;
use regex::{Captures, Regex};
use std::{
    env, fs, io,
    path::{self, Path},
};
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
/// **Signature:** `include_data_url!(path, type)`
///
/// - **`path`:** A string literal. Interpreted as a path relative to the crate root, not the current file. The path must be within the crate as determined by `absolute(path).starts_with(cwd)`.
/// - **`type`:** A string literal. Representing the MIME type of the data that will be inlined. Examples: `"text/plain"`, `"image/png"`, `"image/jpeg"`.
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
/// ![](
#[doc = remote_jcbhmr_include_data_url::include_data_url!("./src/cat-screenshot.png", "image/png")]
/// )
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

    if !path::absolute(&path)
        .unwrap()
        .starts_with(env::current_dir().unwrap())
    {
        panic!("{} not in crate directory", &path);
    }

    let data = fs_err::read(&path).unwrap();
    let encoded = BASE64_STANDARD.encode(&data);
    let data_url = format!("data:{};base64,{}", type_, encoded);

    quote!(#data_url).into()
}
