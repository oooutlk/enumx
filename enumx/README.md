# Motivation

Emulating structural enums in Rust.

# Usage

1. Add this crate to Cargo.toml

`Cargo.toml`:

```toml
[dependencies.enumx]
version = "0.3.1"
```

`src/lib.rs`:

```rust
use enumx::*;
```

2. Use `Enum!(A,B,..)` to define a structural enum, the variants of which are
A, B, .. etc.

```rust
let a: Enum!(i32,String) = 42.into_enum();

let b = <Enum!(i32,String,bool)>::from_variant( "the answer".to_owned() );
```

3. Use `.from_enumx()`/`.into_enumx()` to convert between structural enums
and/or their variants.

```rust
let c: Enum!(i32,String,bool) = 42.into_enumx();

let d: Enum!(i32,String,bool) = a.into_enumx();

let e = <Enum!(i32,String,bool)>::from_enumx( 42 );

let f = <Enum!(i32,bool,String)>::from_enumx( e );
```

4. Use `#[ty_pat] match` to do pattern matching against an `Enum!(A, B, ..)`,
the arms of which are not variants but types A, B, .. etc. The `fn` containing
the match expression must be tagged `#[enumx]`.

```rust
#[enumx] fn foo( input: Enum!(String,i32) ) {
    #[ty_pat] match input {
        String(s) => println!( "it's string:{}", s ),
        i32(i) => println!( "it's i32:{}", i ),
    }
}
```

- Use `#[ty_pat(gen_variants)]` to generate missing types in `Enum!()`

```rust
#[enumx] fn foo( input: Enum!(String,i32) ) -> Enum!(String,i32) {
    #[ty_pat(gen_variants)] match input {
        i32(i) => (i+1).into_enumx(),
        // generated arm: String(s) => s.into_enumx(),
    }
}
```

- Use `#[ty_pat(gen A,B,..)]` to generate A,B,.. etc

```rust
#[enumx] fn foo( input: Enum!(String,i32) ) -> Enum!(String,i32) {
    #[ty_pat(gen String)] match input {
        i32(i) => (i+1).into_enumx(),
        // generated arm: String(s) => s.into_enumx(),
    }
}
```

- Use `TyPat` to wrap types that are not paths, e.g. references, (), in a
`#[ty_pat] match`'s arm:

```rust
#[enumx] fn bar( input: Enum!(&'static str,i32) ) {
    #[ty_pat] match input {
        TyPat::<&'static str>(s) => println!( "it's static str:{}", s ),
        i32(i) => println!( "it's i32:{}", i ),
    }
}
```

5. Use `#[derive( EnumX )]` to make user-defined enums exchangable from/to other
`Enum!()`s or `#[derive(EnumX)]` enums.

```rust
#[derive( EnumX )]
enum Info {
    Code(i32),
    Text(&'static str),
}

let info: Info = 42.into_enum();

let a: Enum!(&'static str, i32) = "a".into_enum();

let info: Info = a.into_enumx();

let b: Enum!(&'static str, i32) = info.into_enumx();
```

**Notice**

All the features provided by this crate work with stable Rust, including
`#[enumx]` closures/let-bindings and `#[ty_pat] match`.

# License

Licensed under MIT.
