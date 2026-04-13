// #![doc(html_favicon_url = include_data_url!("./src/favicon.ico", "image/x-icon"))]
// #![doc(html_logo_url = include_data_url!("./src/logo.png", "image/png"))]
//! TODO

use base64::prelude::*;
use derive_more::{From, Into};
use quote::quote;
use std::fs;
use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// A procedural macro that includes the contents of a file as a `data:` URL
/// string literal. Similar to [`include_str!`].
///
/// Useful for composing with [`#[doc]`](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html) where a non-`file:` URL is expected but
/// the author wants to reference a local file.
///
/// # Examples
///
/// Set the [rustdoc](https://doc.rust-lang.org/rustdoc/) favicon and logo to a `data:` URL generated from crate-local assets.
///
/// <div><code>lib.rs</code></div>
///
/// ```
/// # use jcbhmr_include_data_url::include_data_url;
/// #![doc(html_favicon_url = include_data_url!("./src/favicon.ico", "image/x-icon"))]
/// #![doc(html_logo_url = include_data_url!("./src/logo.png", "image/png"))]
/// ```
///
/// Embed a crate-local image as a `data:` URL in a documentation comment.
///
/// ```
/// # use jcbhmr_include_data_url::include_data_url;
/// /// ![](
/// #[doc = include_data_url!("./src/cat.jpg", "image/jpeg")]
/// /// )
/// struct Cat;
/// ```
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

    let data = fs::read(&path).unwrap_or_else(|e| panic!("{:?} should be readable: {}", path, e));
    let encoded = BASE64_STANDARD.encode(&data);
    let data_url = format!("data:{};base64,{}", type_, encoded);

    quote!(#data_url).into()
}
