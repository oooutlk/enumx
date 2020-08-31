# Backtrace

Backtrace is disabled by default. When enabled, locations of error propagation
by `ret!()`, `throw!()` and `?` operator will be stored in the `Err` variant.

## Use `log` feature to enable backtrace.

```toml
[dependencies.cex]
version = "0.5"
features = ["log"]
```

## Use `env_log` feature to enable backtrace if the envirnoment variable
`RUST_BACKTRACE` is 1 or "full".

```toml
[dependencies.cex]
version = "0.5"
features = ["env_log"]
```

## Use `pretty_log` feature to pretty-print the frames, as if "{:#?}" were used.

```toml
[dependencies.cex]
version = "0.5"
features = ["log","pretty_log"]
# or features = ["env_log","pretty_log"]
```

```rust,no_run
use enumx::export::*;
use enumx::predefined::*;
use cex::*;

#[cex]
pub fn foo() -> Result!( () throws () ) {
    throw!( () );
}

#[cex]
pub fn bar() -> Result!( () throws () ) {
    ret!( foo()? );
}

fn main() {
    bar().unwrap();
}
```

The output is similar as follows:

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: _0(Log {
    error: (),
    agent: [
        Frame {
            module: "my_program",
            file: "src/main.rs",
            line: 5,
            column: 13,
            info: Some(
                "throw!(())",
            ),
        },
        Frame {
            module: "my_program",
            file: "src/main.rs",
            line: 10,
            column: 11,
            info: Some(
                "foo()",
            ),
        },
    ],
})', src/main.rs:14:5
```

## Forward log features to cex crate

```toml
[features]
log = ["cex/log"]
env_log = ["cex/env_log"]
pretty_log = ["cex/pretty_log"]
```

## `ret!()`/`throw!()` could have the second argument as a customized log item.

```rust,no_run
ret!( expr, || frame!( "expect an ok value" ));
throw!( expr, || frame!( "oops" ));
```

Even if backtrace is disabled at compile time, these will compile. The second
argument just has no effect.
