# delegate-match

[![Crates.io](https://img.shields.io/crates/v/delegate-match)](https://crates.io/crates/delegate-match)
[![Docs.rs](https://docs.rs/delegate-match/badge.svg)](https://docs.rs/delegate-match)
[![Rust Version](https://img.shields.io/badge/MSRV-1.81.0-blue)](https://github.com/chipnertkj/delegate-match/blob/main/Cargo.toml)
[![Build](https://github.com/chipnertkj/delegate-match/actions/workflows/ci.yml/badge.svg)](https://github.com/chipnertkj/delegate-match/actions/workflows/ci.yml)

Convenience macro for writing grouped `match` arms for different underlying types.

Writing repetitive `match` arms for enumerations (or other pattern-matching
constructs) &mdash; especially when types, but not the API, differ &mdash;
can quickly become boilerplate. `delegate_match!` lets you list
several patterns up-front once and then re-uses a single body for each
of them, automatically expanding into equivalent ordinary Rust code.

## Examples

### Delegating to the same code for multiple enum variants

```rust
use delegate_match::delegate_match;

enum MouseEvent { Scroll(i16, i16), Position(i32, i32) }
let ev = MouseEvent::Scroll(10, 20);

delegate_match! {
    match ev {
        // This expands to two individual arms.
        MouseEvent::{ Scroll, Position }(x, y) => {
            println!("mouse event: $entry_pat → ({x}, {y})")
        }
    }
}
```

### Using placeholders

```rust
use delegate_match::delegate_match;

enum Msg { Ping, Log }
let msg = Msg::Log;

delegate_match! {
    match msg {
        // Outputs "🏓 Ping" or "📝 Log" depending on the variant.
        Msg::{ Ping: "🏓", Log: "📝" } => {
            // `$assoc_ts` and `$entry_pat` are placeholders substituted at compile time.
            // They are substituted for every entry *before the code is type-checked*,
            // and they may appear in the following places:
            //   - inside the delegate arm pattern (if present),
            //   - inside the match arm guard expression (if present),
            //   - inside the arm body expression.
            println!("{} {}", $assoc_ts, stringify!($entry_pat))
        }
    }
}
```

### Examples in tests/

See [tests/](https://github.com/chipnertkj/delegate-match/tree/main/tests) for more usage examples. These are verified by the CI to compile and execute successfully.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/chipnertkj/delegate-match/tree/main/LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://github.com/chipnertkj/delegate-match/tree/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

See [COPYRIGHT](https://github.com/chipnertkj/delegate-match/tree/main/COPYRIGHT) for more details.
