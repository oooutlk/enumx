# "Type as Pattern" makes sense for narrowing

Suppose two functions which returns "checked exceptions":

```rust,no_run
#[cex] fn foo() -> Result!( () throws A,B );

#[cex] fn bar() -> Result!( () throws A,B,C );
```

If we call `foo()` in `bar()`, the errors in `foo()` may be collected into
`bar()`'s error type `Enum!(A,B,C)`, which is "wider" than `foo()`'s error type 
`Enum!(A,B)`. Usually `?` could be used for convenience, propagating errors
without the need of writing a match expression to handle.

In the contrast, if we call `bar()` in `foo()`, it is not possible for `?`to
propagate the errors from `bar()` because `foo()`'s error type is "narrower"
than `bar()`'s. We must write some match expression and will meet the
[issues](./typat.md).

The `#[ty_pat]` attribute is enabled inside a `#[cex]` tagged function or
closure, to address these issues and make propagating convenient again.

## Use `#[ty_pat] match`

to map errors returned by `#[cex]` functions or closures:

```rust,no_run
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

## Use `TyPat`

to wrap types that are not paths, e.g. references, (), in a `#[ty_pat] match`'s
arm:

```rust,no_run
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

## Use `#[ty_pat(gen_throws)] match`

to automatically propagate errors enumerated in throws:

```rust,no_run
#[cex] fn foo() -> Result!( i32 throws String, SomeError ) {/**/}

#[cex] fn bar() -> Result!( i32 throws String ) {
    foo().or_else( |err| #[ty_pat(gen_throws)] match err {
        SomeError => ret!(0),
        // generated arm: String(s) => throw!(s),
    })
}
```

## Use `#[ty_pat(gen A,B,..)] match`

to automatically propagate errors A,B,.. enumerated in the attribute:

```rust,no_run
#[cex] fn foo() -> Result!( i32 throws String, SomeError ) {/**/}

#[cex] fn bar() -> Result!( i32 throws String ) {
    foo().or_else( |err| #[ty_pat(gen String)] match err {
        SomeError => ret!(0),
        // generated arm: String(s) => throw!(s),
    })
}
```
