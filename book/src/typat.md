# Type as Pattern

The variant names of ad-hoc enums are not so attractive: `_0`,`_1`,`_2`..etc.
There are some issues when these names are used in match expressions:

1. They are ugly and meaningless.
  The numbers do not reflect the types.

2. Subject to changes of ad-hoc enum.
  For instance, changing `Enum!(Alpha,Gamma)` to `Enum!(Alpha,Beta,Gamma)` will
break the arms matching `_1`.

This library provides a feature so called "type as pattern", which extends the
syntax of match expressions to accept variant type names in arm's pattern.

## Use `#[ty_pat] match`

to do pattern matching against an `Enum!(A, B, ..)`,
the arms of which are not variants but types A, B, .. etc. The `fn` containing
the match expression must be tagged `#[enumx]`.

```rust,no_run
#[enumx] fn foo( input: Enum!(String,i32) ) {
    #[ty_pat] match input {
        String(s) => println!( "it's string:{}", s ),
        i32(i) => println!( "it's i32:{}", i ),
    }
}
```

## Use `#[ty_pat(gen_variants)]`

to generate missing types in `Enum!()`:

```rust,no_run
#[enumx] fn foo( input: Enum!(String,i32) ) -> Enum!(String,i32) {
    #[ty_pat(gen_variants)] match input {
        i32(i) => (i+1).exchange_into(),
        // generated arm: String(s) => s.exchange_into(),
    }
}
```

## Use `#[ty_pat(gen A,B,..)]`

to generate A,B,.. etc:

```rust,no_run
#[enumx] fn foo( input: Enum!(String,i32) ) -> Enum!(String,i32) {
    #[ty_pat(gen String)] match input {
        i32(i) => (i+1).exchange_into(),
        // generated arm: String(s) => s.exchange_into(),
    }
}
```

## Use `TyPat`

to wrap types that are not paths, e.g. references, (), in a `#[ty_pat] match`'s
arm:

```rust,no_run
#[enumx] fn bar( input: Enum!(&'static str,i32) ) {
    #[ty_pat] match input {
        TyPat::<&'static str>(s) => println!( "it's static str:{}", s ),
        i32(i) => println!( "it's i32:{}", i ),
    }
}
```
