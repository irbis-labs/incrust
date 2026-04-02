# incrust-format

Composable `Display` primitives for the incrust ecosystem. Usable as a standalone crate.

## API

### `StrBuffer<const N: usize>`

Fixed-size string buffer backed by a `[u8; N]` array. Useful for buffering partial output
in streaming formatters without heap allocation.

### `DisplayIterator` trait

Extension trait for iterators of `Display` items:

- `display_concat()` — concatenate all items
- `display_join(separator)` — join items with a separator

### Re-exports from `smart-string`

- `DisplayExt` — composable `Display` adapter methods
- `PascalString<N>` — inline string with length prefix

## Usage

```rust
use incrust_format::prelude::*;

let items = vec!["a", "b", "c"];
let joined = items.display_join(", ").to_string();
assert_eq!(joined, "a, b, c");
```

## License

MIT / Apache-2.0
