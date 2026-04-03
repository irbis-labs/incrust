![](https://img.shields.io/crates/l/incrust.svg)
![Minimum Rust version: 1.85](https://img.shields.io/badge/MSRV-1.85-blue.svg)

## {% Incrust %}

> Incrust is a template engine for Rust, originally inspired by Jinja2.

The engine is not yet active in this version. Template compilation, AST, and
evaluation will be designed here in future iterations.

Formatting and escaping primitives that previously lived in this workspace
(`incrust-format`, `incrust-filters`) have been extracted into the standalone
[`smart-format`](https://crates.io/crates/smart-format) crate.

## Development

### Quality gates

```bash
cargo +nightly fmt -- --check
cargo check
cargo clippy -- -D warnings
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
