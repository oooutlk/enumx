# Summary

Add types, traits, macros and syntactic sugar for working with the `Result` type which models checked exception constructs in error handling.

The new constructs are:

* A `Cex` struct in `Result<T, Cex<Enum!( Err1, Err2, ...)>>` for simulating `throws` in function signatures.

* A set of ad-hoc enum notated by `Enum!()` that can be converted to each other if compatible in variant types.

* An optional `??` pseudo operator and its equivalent APIs, names of which start with "throw" or "may_throw", for explicityly propagating "checked exceptions" from a plain error.

* An optional `???` pseudo operator and its equivalent APIs, names of which start with "rethrow" or "may_rethrow", for explicityly propagating "checked exceptions" from another "checked exception".

# Motivation and overview

Using `Result` and `?` operator in error handling is very convenient, except for the requirement of handwritten boilerplate code for error type definitions and convertions, or using trait object. We would like to solve it, not to force users to implement "the trait" for their error types, but automatically generate type definitions and convertions.

We can accomplish this by adding constructs which mimic union types and checked exception of other languages, while implement them in typically Rustic fashion. Their meaning can be specified by two existing rust crates, [EnumX](https://crates.io/crates/enumx) and [CeX](https://crates.io/crates/cex), but with some limitations, and a few features are not implemented at the moment.

These constructs are strict landed in "user mode rust", without any magic in the compiler. Apart from the issue of pseudo operators, the legality and behavior of all currently existing Rust syntax is entirely unaffected.

By "checked exceptions", fo now, we essentially just mean a `Cex`'s `error` field that is the `Err` variant of a `Result`. And "plain error" means the variant of an `Enum!()`/`#[derive(Exchange)] enum`.

When a checked exception was `throw`ed from the callee, the caller can simply `rethrow`s it, or does actual error handling on `match`ing its variants. If failed again, the caller will `throw` some checked exception which may or may not be the origin one.

## Unconditionally throw/rethrow

  When talking about `throw`, we mean "converting a plain error to a checked exception and do early exit".

  A set of macros for simulating `throw` keywords are `throw!()`, `throw_log!()`, `throw_ex!()` and `throw_log_ex!()`.

  When talking about `rethrow`, we mean "converting a checked exception to another one and do early exit".

  A set of macros for simulating `rethrow` keywords are `rethrow!()`, `rethrow_log!()`, `rethrow_ex!()`, `rethrow_log_ex!()`, `rethrow_named!()` and `rethrow_log_named!()`..

## The conditional throw API, and `??` pseudo operator

  Similar with using `?` operator to propagating a plain error, `??` propagates a checked exception. 

  The following is an example of a function that reads a `u32` value from a file.

  ```rust
  #[cex]
  fn read_u32( filename: &'static str )
      -> Result<u32, Cex<Enum!( std::io::Error, std::num::ParseIntError )>>
  {
      use std::io::Read;
  
      let mut f = std::fs::File::open( filename )??;
      let mut s = String::new();
      f.read_to_string( &mut s )??;
      let number = s.trim().parse::<u32>()
                    .may_throw_log( ||log!( "fail in parsing {} to u32", s.trim() ))?; // don't double `?`
      Ok( number )
  }
  ```

  The `??` pseudo operator reads as "may throw?", which is activated by the proc-macro attribute `#[cex]`.

  Its equivalent APIs are `may_throw()?`, `may_throw_log()?`, `may_throw_ex()?` or `may_throw_log_ex()?`.

## The conditional rethrow API, and `???` pseudo operator

  Similar with using `??` pseudo operator to propagating a checked exception from a plain error, `???` propagates a checked exception from another one.

  The following is an example of a function that reads three `u32` values a, b, and c from three files, and checks if a * b == c.

  ```rust
  #[derive( Debug, PartialEq, Eq )]
  struct MulOverflow( u32, u32 );
  
  #[cex]
  fn a_mul_b_eq_c( file_a: &'static str, file_b: &'static str, file_c : &'static str )
      -> Result<bool, Cex<Enum!( std::io::Error, std::num::ParseIntError, MulOverflow )>>
  {
      let a = read_u32( file_a )???;
  
      let b = match read_u32( file_b ) {
          Ok(  value ) => value,
          Err( cex   ) => {
              if a == 0 {
                  0 // 0 * b == 0, no matter what b is.
              } else {
                  rethrow_log!( cex );
              }
          },
      };
   
      let c = match read_u32( file_c ) {
          Ok(  value ) => value,
          Err( cex   ) => match cex.error {
              Enum2::_0( _   ) => 0, // default to 0 if file is missing.
              Enum2::_1( err ) => throw!( err ),
          },
      };
  
      a.checked_mul( b )
       .ok_or( MulOverflow(a,b) )
       .may_throw_log( ||log!( "u32 overflow: {} * {}", a, b ))
       .map( |result| result == c )
  }
  ```

  The `???` pseudo operator reads as "may rethrow?", which is activated by the proc-macro attribute `#[cex]`.

  Its equivalent APIs are `may_rethrow()`, `may_rethrow_log()`, `may_rethrow_ex()`, `may_rethrow_log_ex()`, `may_rethrow_named()` and `may_rethrow_log_named()`.

## `throws` in function signatures

  The two functions listed above are in the form of `#[cex] fn(args) -> Result<T, Cex<Enum!( A, B, C )>>`, which can be translated straightforward to `fn(args) -> T, throws A, B, C` if Rust supports this kind of syntax.

## Ad-hoc enum

  `Enum!( std::io::Error, std::num::ParseIntError )` produces an ad-hoc enum composed by two variants. Its variants are named as `_0` and `_1`, when referenced later, we can simply use `Enum2::_0` and `Enum2::_1`.

  We defines a new error type( a plain error ) `MulOverflow` for function `a_mul_b_eq_c()`. Note that it is not an `enum` but a `struct` without `impl From`s. Plain errors will be summarized by the ad-hoc enum wrapper.

## How an checked exception are checked

  A checked exception as the `Cex`'s `error` field are either an ad-hoc enum or a `Exchange`able enum, which must be exhaustively checked in a `match`.

## Log and backtracing support

  When (re)`throw`ing a checked exception, an optional backtrace information can be logged. The backtrace log includes the module, file, line and column of the throw point, and an optional info string.

  If we do not want to log anything, use `throw()`, `rethrow()`, `may_throw()`, `may_rethrow()`, and the corresponding ones with "_ex" suffix in their function names.

  If we want to track only the throw point but nothing else, use `throw_log( err, log!() )`, `rethrow_log( err, log!() )`, `may_throw_log( err, log!() )`, `may_rethrow_log( err, log!() )`, and the corresponding ones with "_ex" suffix in their function names.

  If we want to track the throw point, and an additional `info`, the format string and the `info` can be put in the arguments of `log!()` macro, which behaves like `std::format!()` but constructs a `Log`.

  An example demonstrating what backtrace looks like:

  ```rust
      assert_eq!( a_mul_b_eq_c( "src/test/3", "src/test/not_num", "src/test/21" ).map_err( |cex| format!( "{:#?}", cex )),
                  Err( String::from( r#"Cex {
      error: _1(
          ParseIntError {
              kind: InvalidDigit
          }
      ),
      logs: [
          Log {
              module: "cex::test::sugar",
              file: "cex/src/test/sugar.rs",
              line: 19,
              column: 35,
              info: Some(
                  "fail in parsing not-a-number to u32"
              )
          },
          Log {
              module: "cex::test::sugar",
              file: "cex/src/test/sugar.rs",
              line: 38,
              column: 17,
              info: None
          }
      ]
  }"# ))
      );
  ```

## Named checked exception

  It is convenient to write an ad-hoc enum but inconvenient to access its variants via the unreadable names such as `_0`. Named `Exchange`able enum comes to rescue, at the cost of an explicit user-defined type with a `#[derive(Exchange)]` attribute.

  The `read_u32()` and `a_mul_b_eq_c()` examples could be rewriten to use named checked exception:

  ```rust
  #[ derive( Exchange, Debug )]
  enum ReadU32Error {
      IO( std::io::Error ),
      Parse( std::num::ParseIntError ),
  }
  
  fn read_u32( filename: &'static str ) -> Result<u32, Cex<ReadU32Error>> {
    /* body omitted */
  }

  #[derive( Debug, PartialEq, Eq )]
  struct MulOverflow( u32, u32 );
  
  #[ derive( Exchange, Debug )]
  enum AMulBEqCError {
      IO( std::io::Error ),
      Parse( std::num::ParseIntError ),
      Overflow( MulOverflow ),
  }
  
  fn a_mul_b_eq_c( file_a: &'static str, file_b: &'static str, file_c : &'static str )
      -> Result<bool, Cex<AMulBEqCError>>
  {
    /* omitted */

    let c = match read_u32( file_c ) {
        Ok(  value ) => value,
        Err( cex   ) => match cex.error { // variants has readable names
            ReadU32Error::IO(    _   ) => 0, // default to 0 if file is missing.
            ReadU32Error::Parse( err ) => throw!( err ),
        },
    };

    /* omitted */
  }
  ```

  Those `??`/`???` equivalent APIs with a `_ex` name suffix mentioned previously, works for the situation that both callee and caller using named checked exceptions.

  The APIs with a `_named` name suffix mentioned previously, works for the situation that callee using ad-hoc checked exception while caller using named one.

# Detailed design

The checked exception simulation proposed in this article is essentially a filling the blank in "Exception type upcasting" section in [RFC 243](https://github.com/rust-lang/rfcs/blob/master/text/0243-trait-based-exception-handling.md#exception-type-upcasting).

To automatically generate "type upcasting", some kind of "union types" must be implemented. In the upcasting, types are categorized into one of the three:

1. variant types

  - converted to #2 and #3 via `IntoEnum` trait.

2. ad-hoc `Enum!()` types

  - constructed from #1 via `FromVariant` trait.

  - converted between another one via `FromEnumX`/`IntoEnumX` trait.

  - converted to #3 of the same variant types via standard `From`/`Into` trait.

3. named `Exchange`able types

  - constructed from #1 via `FromVariant` trait.

  - converted to #2 of the same variant types via standard `From`/`Into` trait.

  - converted between another one via `ExchangeFrom`/`ExchangeInto` trait.

All the traits mentioned above utilize phantom `Index`/`Indices` types to do type inferences, for which standard `From`/`Into` traits have no rooms to place in generics. It is the root cause why we distinguish ad-hoc enums with named exchangeable ones, `throw` with `rethrow`, `?` with `??`/`???` etc.

## Throw/Rethrow macros

  They are a thin wrapper of the `Throw`/`Rethrow`'s methods, e.g. `throw!( error )` is `return error.throw()`;

## `??`/`???` pseudo operators and their equivalents

  They combine `map_err()` and `MayThrow`/`MayRethrow`'s methods, e.g. `expr??` or `expr.may_throw_log( log!() )` is `expr.map_err( |err| Cex{ error: err.into_enum(), logs: vec![ log ]})`.

  In order to use `??`/`???`, the user must tag their functions with `#[cex]` attribute. Currently, `expr??` will be translated in `expr.may_throw_log( log!() )` and `expr???` will be translated in `expr.may_rethrow_log( log!() )`.

## Feature gates

  EnumX's feature `enum32` supports enums composed of up to 32 variants. By default enums of up to 16 variants are supported.

# Drawbacks

1. The `??`/`???` pseudo operators change the meaning of exising Rust syntax. However `??` or `???` seems not to be frequent used( if ever used ), and `#[cex]` indicates the change.

2. Although well-defined and clear in their names, the prefix "re" and suffix "ex"/"named" seems verbose and complex, especially for beginners. If we find a way to do type upcasting in the manner of standard `From`/`Into`, they could be omitted for brevity.

3. As mentioned previously, union types in EnumX project does not support enums composed of arbitrary number of variants. By default the maximum is 16, and can be increased to 32.

# Rationale and alternatives

Checked exception simulation in rust have several advantages:

1. keeps users from writing boilerplate code for type definitions and convertions.

2. clear about all the possible error types by checking the function signature.

3. do not force users to `impl` certain trait to work.

The debatable topics on checked exception are:

1. Checked exceptions inappropriately expose the implementation details.

2. Unstable method signatures. 

# Prior art

The EnumX v0.2 is inspired by [`frunk_core::coproduct`](https://docs.rs/frunk_core/0.2.2/frunk_core/coproduct/index.html), which provides another ad-hoc enum implementation.

# Unresolved questions

1. Should or can the type upcasting implemented finally in the manner of standard `From`/`Into`?

The EnumX v0.1 did it this way at the cost of unable to work for enums with generic arguments,while EnumX v0.2 uses phantom index types to work for it at the cost of using `From`/`Into`.

2. Should the union types implemented not in flattern `enum`s but in nested car/cdr manner, similar with `frunk_core::coproduct`?

Nested car/cdr enum can deal with arbitrary variant numbers, at the cost of pattern matching syntax support associated with Rust `enum`.

3. Should the backtrace enabling/disabling at runtime by some environment variable?

# Future possibilities

Add `cex!{}` proc-macro to mimic `throws` keywords and inline type definition.

```rust
cex! {
    pub fn foo( args ) -> T throws enum FooErr{ Code(i32), Text(String) } { 
        throw 0xdeadbeef;
    }
}
```

will be translated to

```rust
#[derive(Exchange)]
pub enum FooErr{ Code(i32), Text(String) }

pub fn foo( args ) -> Result<T, Cex< enum FooErr{ Code(i32), Text(String) }>> { 
        throw!( 0xdeadbeef );
    }
}
```

# License

Licensed under MIT.
