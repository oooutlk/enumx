# Purpose

This is a proof of concept of project aiming at implementing "checked exception" like error handling in Rust via proc-macro.

It is based on `enumx` crate. See `../enumx/README.md` for more.

# Naming

It provides `CeX` structures, that is read as "Concrete Error Exchange" or "Checked EXception", at your will.

A `CeX` is essentially a composition of `EnumX` for exchange, and a `Backtrace` for stack backtracing.

`CeX`es composed of x errors are named as `Ce1`, `Ce2`, ... etc.

# type converting rules:

1. Each error type in a `CeX` is convertable to this `CeX`.

2. An `CeX` T is convertable to another `CeX` U if all errors in T are in U.

Inside a function with `#[cex]` attribute and returning `Result<_,CeX<_,...>`, the code can return or try(`?`) any type that is convertable to the `CeX` type in the `Err` part of the function result type.

However, **explicit `?` or `return` are required**.

If you do not want backtracing, just use `Result<_,EnumX<_,...>` instead.

# Examples

- A function checking 2 errors.

  ```rust
  #[cex]
  fn f2( quit: usize ) -> Result<(), Ce2<String, i32>> {
      match quit {
          0 => Err("runtime error".to_string())?,
          1 => return Err(0xdead),
          _ => Ok(()),
      }
  }
  assert_eq!( f2(0), Err( Ce2::_0( "runtime error".to_string() ))); 
  assert_eq!( f2(1), Err( Ce2::_1( 0xdead )));
  assert_eq!( f2(2), Ok(()) );
  ```

- Some 2-errors `CeX` that is convertable to 3-errors `CeX`.

  ```rust
  #[cex]
  fn f3( quit: usize ) -> Result<(), Ce3<String,i32,bool>> {
      match quit {
          0 | 1 => Ok(f2(quit)?),
          2 => Err(false)?,
          _ => Ok(()),
      }
  }
  assert_eq!( f3(0), Err( Ce3::_0( "runtime error".to_string() )));
  assert_eq!( f3(1), Err( Ce3::_1( 0xdead )));
  assert_eq!( f3(2), Err( Ce3::_2( false )));
  assert_eq!( f3(3), Ok(()) );
  ```

- **Errors' order does not matter**.

  ```rust
  #[cex]
  fn g3( variant_index: usize ) -> Result<(), Enum3<i32,bool,String>> {
      match variant_index {
          0|1 => Ok( f2( variant_index )? ),
            _ => return f3( variant_index ),
      }
  }
  assert_eq!( g3(0), Err( Enum3::_2( "runtime error".to_string() )));
  assert_eq!( g3(1), Err( Enum3::_0( 0xdead )));
  assert_eq!( g3(2), Err( Enum3::_1( false )));
  ```

- Those who are not interested in "checked exceptions" can work with the libs that using "checked exceptions".

  ```rust
  fn h( quit: usize ) -> Result<(), String> {
      match f3( quit ) {
          Ok(_) => Ok(()),
          Err( err ) => {
              match err {
                  Enum3::_0( string ) => Err( string ),
                  Enum3::_1( errno  ) => Err( format!( "errno: 0x{:x}", errno )),
                  Enum3::_2( flag   ) => Err( format!( "flag: {}", flag )),
              }
          },
      }
  }
  assert_eq!( h(0), Err( "runtime error".to_string() ));
  assert_eq!( h(1), Err( "errno: 0xdead".to_string() ));
  assert_eq!( h(2), Err( "flag: false".to_string() ));
  assert_eq!( h(3), Ok(()));
  ```

- Backtracing support.

  ```rust
  #[cex]
  fn foo() -> Result<(),Ce1<()>> {
      Err(())?
  }
  if let Err(err) = foo() {
      eprintln!( "{:#?}", err.backtrace );
  }
  ```

  The code listed above will print something like:

  ```text
  Backtrace(
      [
          ThrowPoint {
              line: 110,
              column: 5,
              function: "foo",
              module: "cex_test",
              file: "cex_test/src/lib.rs"
          },
          ThrowPoint {
              line: 115,
              column: 5,
              function: "bar",
              module: "cex_test",
              file: "cex_test/src/lib.rs"
          }
      ]
  )
  ```

# Limitation

- The library users should use concrete error types as the type parameters of CeX, rather than generics.

- **DO NOT use nested `CeX` and expect it be flatterned**.

  Currently this project does nothing to support flatterning.

- Current version supports up to 32 variants in a `CeX`.

  In otherwords, `Ce1`..=`Ce32` is available but `Ce33` but the succeeding enums are not.

  If it is proved that more than comination of 32 errors are useful in practice, they may be supported in later version.

  Notice that the compile time will grow in `O( n*n )` `impl`s.

- Some sort of identifiers are reserved for implementation usage.

  `cex`/`cex_derive`crates reserve identifiers starting with `__CeX`, `__cex` and all that `enumx`/`enum_derive` reserves.

# License

Licensed under MIT.
