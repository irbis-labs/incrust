# incrust-filters

Composable streaming escape and transform filters for Rust, built on the `Display` trait.
Usable as a standalone crate — does not require the incrust template engine.

## Features

All filter families are feature-gated (default: none enabled). Enable individually or use `full`:

| Feature | Trait | Methods |
|---------|-------|---------|
| `FilterHtml` | `FilterHtml` | `html_escape()`, `html_escape_strict()`, `html_unescape()`, `html_attribute()` |
| `FilterUrl` | `FilterUrl` | `url_escape()` |
| `FilterXml` | `FilterXml` | `xml_c_data()` |
| `FilterCase` | `FilterCase` | `capitalize()`, `lowercase()`, `uppercase()` |

```toml
[dependencies]
incrust-filters = { version = "0.6", features = ["FilterHtml", "FilterUrl"] }
```

## Usage

```rust
use incrust_filters::FilterHtml;

let escaped = "Tom & Jerry".html_escape().to_string();
assert_eq!(escaped, "Tom &amp; Jerry");
```

Filters compose via `Display` chaining — no intermediate `String` allocations.

## Design

- Bitset-based O(1) escape lookup (`u128` bitmask per escape table)
- Streaming: each filter wraps `fmt::Display`, composing through `fmt::Formatter`
- Zero-copy: no heap allocation for the escape pass itself

## License

MIT / Apache-2.0
