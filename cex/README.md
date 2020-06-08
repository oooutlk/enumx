# Motivation

Emulating checked exceptions in Rust.

Prefer smaller enum that holds errors that may haven been encountered in the
function, rather than a big enum that holds the various possible errors that may
have been encountered at any point anywhere in the crate.

# Usage

1. Add this crate to Cargo.toml, and enable any features you want

`Cargo.toml`:

```toml
[dependencies.cex]
version = "0.4"
features = ["log","pretty_log"]
```

`src/lib.rs`:

```rust
use cex::*;
```

2. Use `Result!()` to enumerate the possible error types

- in function signature:

```rust
#[cex] fn throws_never() -> Result!(i32) {/**/}

struct SomeError;

#[cex] fn foo() -> Result!( i32 throws String, &'static str, SomeError ) {/**/}
```

- in closure's signature:

```rust
#[cex] fn foo() {
    let _f = #[cex] || -> Result!( i32 throws String ) {/**/}
}
```

- in the type annotation of a local let-binding:

```rust
#[cex] fn foo() {
    #[cex] let v: Result!( i32 throws String ) = try {/**/};
}
```

3. Use `ret!()`/`throw!()` to return a result.

- use `ret!(expr)` to return an `Ok` expr

```rust
#[cex] fn foo() -> Result!( i32 throws String ) {
    ret!( 42 ); // Ok-wrapping
}
```

- use `ret!(expr)` to return a `Result` expr

```rust
#[cex] fn foo() -> Result!( i32 throws String ) {
    ret!( Ok( 42 ));
    // or ret!( Err( String::from( "oops" )))
}
```

- use `throw!(expr)` to return an `Err`, the value of which is converted from
the expr.

```rust
#[cex] fn foo() -> Result!( i32 throws String, SomeError ) {
    throw!( String::from( "oops" ))
    // or throw!( SomeError )
}

#[cex] fn bar() -> Result!( i32 throws String, &'static str, SomeError ) {
    match foo() {
        Ok(v) => ret!(v),
        Err(e) => throw!(e), // all errors in foo()'s throws are in bar()'s
    }
}

// equivalent to bar()
#[cex] fn baz() -> Result!( i32 throws String, &'static str, SomeError ) {
    ret!( foo()? ) // of course you can use `?` to propagate errors
}
```

4. Use `#[ty_pat] match` to map errors returned by `#[cex]` functions or
closures.

```rust
#[cex] fn foo() -> Result!( () throws String, SomeError ) {/**/}

#[cex] fn bar() {
    if let Err( err ) = foo() {
        #[ty_pat] match err {
            String( s ) => println!( "foo's error:{}", s ),
            SomeError => println!( "foo's error: SomeError" ),
        }
    }
}
```

- use `TyPat` to wrap types that are not paths, e.g. references, (), in a
`#[ty_pat] match`'s arm:

```rust
#[cex] fn foo() -> Result!( i32 throws &'static str, SomeError ) {/**/}

#[cex] fn bar() {
    if let Err( err ) = foo() {
        #[ty_pat] match err {
            TyPat::<&'static str>( s ) => println!( "foo's error:{}", s ),
            SomeError => println!( "foo's error: SomeError" ),
        }
    }
}
```

- use `#[ty_pat(gen_throws)] match` to automatically propagate errors enumerated
in throws:

```rust
#[cex] fn foo() -> Result!( i32 throws String, SomeError ) {/**/}

#[cex] fn bar() -> Result!( i32 throws String ) {
    foo().or_else( |err| #[ty_pat(gen_throws)] match err {
        SomeError => ret!(0),
        // generated arm: String(s) => throw!(s),
    })
}
```

- use `#[ty_pat(gen A,B,..)] match` to automatically propagate errors A,B,..
enumerated in the attribute:

```rust
#[cex] fn foo() -> Result!( i32 throws String, SomeError ) {/**/}

#[cex] fn bar() -> Result!( i32 throws String ) {
    foo().or_else( |err| #[ty_pat(gen String)] match err {
        SomeError => ret!(0),
        // generated arm: String(s) => throw!(s),
    })
}
```

# Backtrace

Backtrace is disabled by default. When enabled, locations of error propagation
by `ret!()`, `throw!()` and `?` operator will be stored in the `Err` variant.


- Use `log` feature to enable backtrace.
```toml
[dependencies.cex]
version = "0.4"
features = ["log"]
```

- Use `env_log` feature to enable backtrace if the envirnoment variable
`RUST_BACKTRACE` is 1 or "full".

```toml
[dependencies.cex]
version = "0.4"
features = ["env_log"]
```

- Use `pretty_log` feature to pretty-print the frames, as if "{:#?}" were used.

```toml
[dependencies.cex]
version = "0.4"
features = ["log","pretty_log"]
# or features = ["env_log","pretty_log"]
```

```rust
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

```
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

- Forward log features to cex crate

```toml
[features]
log = ["cex/log"]
env_log = ["cex/env_log"]
pretty_log = ["cex/pretty_log"]
```

- `ret!()`/`throw!()` could have the second argument as a customized log item.

```rust
ret!( expr, || frame!( "expect an ok value" ));
throw!( expr, || frame!( "oops" ));
```

Even if backtrace is disabled at compile time, these will compile. The second
argument just has no effect.

# Summary

A `Result!()` macro, translating `Result!( OkType throws A,B,.. )` into
`Result<OkType, Enum!(A,B,..)>`. The `Enum!(A,B,..)` denotes an enum the variants
of which are A,B,.. , etc. It is usually the return type of functions, closures
or try blocks.

A `throw!( expr )` macro invocation exits the function with Result::Err, the
value of which is converted from expr.

A `ret!( expr )` macro invocation does early return with the value of
Ok( expr ), or expr. In other words, it provides **Ok-wrapping** as needed.

A `#[ty_pat] match` provides "type as pattern matching" feature in the match's
arms, to consume the result returned by functions/closures/try blocks with
`#[cex]` attributes.

The valid forms of `#[ty_pat]` are:

- `#[ty_pat]`. Providing "type as pattern matching" only.

- `#[ty_pat( gen_throws )]`. Besides, auto generating arms that `throw!()`s
errors enumerated in `Result!()`.

- `#[ty_pat( gen A,B,.. )]`. Besides, auto generating arms that `throw!()`s
A,B,.. , etc.

The #[cex] attribute on a function/closure/local let-bingding defines a scope
that may hold a `Result!()` type. The #[cex] scopes could be nested. The inner's
`Result!()` overrides the outer's.

**Notice**

All the features provided by this crate work with stable Rust, including
`#[cex]` closures/let-bindings and `#[ty_pat] match`.

# License

Under MIT.
