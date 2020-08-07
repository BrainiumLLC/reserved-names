# `reserved-names`

The reserved name lists from cargo's [`restricted_names.rs`](https://github.com/rust-lang/cargo/blob/5fe8ab57e2a88ccaaab0821c306203eb19edf8fd/src/cargo/util/restricted_names.rs) in a convenient lil crate!

```rust
assert!(reserved_names::is_reserved("let"));
```
