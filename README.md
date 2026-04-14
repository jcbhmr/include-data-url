# include-data-url

🔗 Like [`include_str!()`](https://doc.rust-lang.org/std/macro.include_str.html), but returns a [`data:` URL](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data)

<table align=center><td>

```rust
/// Look at this cat:
/// 
/// ![](
#[doc = include_data_url!("./src/cat.jpg", "image/jpeg")]
/// )
struct Cat;
```

</table>

## Installation
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

```sh
cargo add jcbhmr-include-data-url
```

## Usage
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

```rust
use jcbhmr_include_data_url::include_data_url;

/// ![](
#[doc = include_data_url!("./src/cat.jpg", "image/jpeg")]
/// )
struct Cat;

fn main() {
    const ASSET: &str = include_data_url!("./src/cat.jpg", "image/jpeg");
    println!("You should navigate to this URL: {}", ASSET);
}
```

## Development
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

```sh
cargo test
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
