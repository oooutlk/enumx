# Motivation

Simulating ad-hoc enums which

1. can be converted between enums that share a common
set of variant types.

2. can implement traits that haven been implemented by all variants.

# Usage

Add this crate to Cargo.toml

`Cargo.toml`:

```toml
enumx = "0.4"
```

Add this if you want to support up to 32 variants:

```toml
features = ["enum32"]
```

`src/lib.rs`:

```rust
use enumx::export::*;
```

If you want to use predefined enum types:

```rust
use enumx::predefined::*;
```

# Features

- "union types" simulation, aka "enum exchange".

- summaries into an enum, the returned values of different types by functions
that returns `impl Trait`.

- macros to help implementing traits for enums the variants of which have all
implemented the traits.

# Documentation

See the [enumx book](https://oooutlk.github.io/enumx/) for more.

# License

Licensed under MIT.
