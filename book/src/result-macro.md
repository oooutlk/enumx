# `Result!()` macro

The syntax of `Result!()` macro is
`Result!( OkType throws Err1, Err2, .. )`, the underlying type of which is
`Result<OkType, Enum!(Err1, Err2, ..)>`. However the `Result!()` macro is
preferred over `Enum!()` because:

1. `Enum!()` is subject to changes on feature of `log`/`env_log`, while
`Result!()` is not.

2. `throws` is cool, shorter and more clear than `Enum!()`.

## Use `Result!()` to enumerate the possible error types

- in function signature:

```rust,no_run
#[cex] fn throws_never() -> Result!(i32) {/**/}

struct SomeError;

#[cex] fn foo() -> Result!( i32 throws String, &'static str, SomeError ) {/**/}
```

- in closure's signature:

```rust,no_run
fn foo() {
    let _f = #[cex] || -> Result!( i32 throws String ) {/**/}
}
```

- in the type annotation of a local let-binding:

```rust,no_run
fn foo() {
    #[cex] let v: Result!( i32 throws String ) = try {/**/};
}
```
