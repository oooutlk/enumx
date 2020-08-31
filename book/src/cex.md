# Checked exception

This library simulates checked exception by enumerating in function's signatures 
every possible error types in [ad-hoc enums](./ad-hoc-enums.md) which are
[`Exchange`-able](./exchange.md) and matched their variants'
[types as patterns](./typat.md).

## Usage

Add this crate to Cargo.toml, and enable any feature you want

`Cargo.toml`:

```toml
enumx = "0.4"
cex = "0.5"
```

`src/lib.rs`:

```rust,no_run
use enumx::export::*;
use enumx::predefined::*; // or use your own enum types at your will.
use cex::*;
```

## Extended syntax:

1. `#[cex]` proc macro attribute for functions/closures/let-bindings returning
checked exceptions.

2. `Result!()` annotates the return type.

3. `ret!()`/`throw!()` for control flow.

4. `#[ty_pat]` for "type as pattern"
