# The `ret!()`/`throw!()` macros

The underlying control flow constructs of these two macros are `return`.
However, `ret!()`/`throw!()` macros are preferred over `return` because:

1. Using `return` is subject to changes on feature of `log`/`env_log`, while
using `ret!()`/`throw!()` are not.

2. `ret!()`/`throw!()` are cool and more clear than `return`.

3. `ret!()` supports Ok-wrapping.

## The syntax of `ret!()`

1. `ret!( ok_value )`, or

2. `ret!( result_value )`

In other words, you can use `ret!()` to return an `Ok` expression:

```rust,no_run
#[cex] fn foo() -> Result!( i32 throws String ) {
    ret!( 42 ); // Ok-wrapping
}
```

or you can use `ret!()` to return a `Result` expression:

```rust,no_run
#[cex] fn foo() -> Result!( i32 throws String ) {
    ret!( Ok( 42 ));
    // or ret!( Err( String::from( "oops" )))
}
```

## The syntax of `throw!()`

is `throws!( err_value )`.

You can use `throw!()` to return an `Err` expression:

```rust,no_run
#[cex] fn foo() -> Result!( i32 throws String, SomeError ) {
    throw!( String::from( "oops" ))
    // or throw!( SomeError )
}
```

Thanks to the power of [`Exchange`](./exchange.md):

```rust,no_run
#[cex] fn bar() -> Result!( i32 throws String, &'static str, SomeError ) {
    match foo() {
        Ok(v) => ret!(v),
        Err(e) => throw!(e), // all errors in foo()'s throws are in bar()'s
    }
}
```

Thanks to the power of `?` which looks like throwing checked exceptions:

```rust,no_run
// equivalent to bar()
#[cex] fn baz() -> Result!( i32 throws String, &'static str, SomeError ) {
    ret!( foo()? ) // of course you can use `?` to propagate errors
}
```
