# Motivation

Emulating checked exceptions in Rust.

# Usage

1. Add this crate to Cargo.toml, and enable any features you want

`Cargo.toml`:

```toml
[dependencies.enumx]
version = "0.4"

[dependencies.cex]
version = "0.5"
```

Add this if you want to support backtrace:

```toml
features = ["log","pretty_log"]
```

`src/lib.rs`:

```rust
use enumx::export::*;
use enumx::predefined::*; // or use your own enum types at your will.
use cex::*;
```

# Features

1. ad-hoc enums as checked exceptions.

2. Backtrace.

3. Type as pattern.

4. Fallback as `impl Trait`.

# Documentation

See the [enumx book](https://oooutlk.github.io/enumx/) for more.

# License

Under MIT.
