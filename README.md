![](https://img.shields.io/crates/l/incrust.svg)
![Minimum Rust version: 1.85](https://img.shields.io/badge/MSRV-1.85-blue.svg)

## {% Incrust %}

> Incrust is a template engine for Rust, originally inspired by Jinja2.

The workspace contains three crates:

| Crate | Purpose | Status |
|-------|---------|--------|
| `incrust-format` | Composable `Display` primitives (`StrBuffer`, `DisplayIterator`, `Join`, `Concat`). Depends on `smart-string`. | Active |
| `incrust-filters` | Streaming escape/transform filters: HTML, URL, XML, case. Feature-gated, usable standalone. | Active |
| `incrust` (engine) | Template compilation and rendering. | Not yet active |

The `filters` and `format` crates are independently useful outside the template engine.

## Crates

### `incrust-filters`

Composable formatting routines built on the `Display` trait. Each filter family is feature-gated:

- **`FilterHtml`** — `html_escape()`, `html_escape_strict()`, `html_unescape()`, `html_attribute()`
- **`FilterUrl`** — `url_escape()` (percent-encoding)
- **`FilterXml`** — `xml_c_data()`
- **`FilterCase`** — `capitalize()`, `lowercase()`, `uppercase()`

Enable features individually or use `full` for all.

### `incrust-format`

Composable `Display` primitives:

- `StrBuffer<N>` — fixed-size string buffer (const-generic)
- `DisplayIterator` trait — `display_concat()`, `display_join()` for iterator formatting
- Re-exports `DisplayExt` and `PascalString` from `smart-string`

## Development

### Quality gates

```bash
cargo +nightly fmt -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-features
cargo clippy --workspace --all-targets -- -D warnings
```

### Git hooks

Enable repo-local hooks (once per clone):

```bash
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit .githooks/pre-push
```

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
