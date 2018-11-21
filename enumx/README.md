# Purpose

This project is a proof of concept of implementing anonymous enum in Rust via proc-macro.

If you are seeking for error handling, perhaps `cex` crate is more suitable for you.

# Naming

Anonymous enums composed of x variants are named as `Enum1`, `Enum2`, ... etc.

Variants are named as `_0`, `_1`, ... etc.

# type converting rules:

1. Each variant type in an `EnumX` is convertable to this `EnumX`.

2. An `EnumX` T is convertable to another `EnumX` U if all variants in T are in U.

Inside a function with `#[enumx]` attribute and returning `EnumX`, the code can return any type that is convertable to the function return type.

However, **explicit `.into()` are required**.

# Examples

- A function returning an anonymous enum composed of 2 variants.

  ```rust
  #[enumx]
  fn f2( variant_index: usize ) -> Enum2<String,i32> {
      match variant_index {
          0 => "runtime error".to_string().into(),
          1 => 0xdead.into(),
          _ => panic!( "variant index out of bounds" ),
      }
  }
  
  assert_eq!( f2(0), Enum2::_0( "runtime error".to_string() ));
  assert_eq!( f2(1), Enum2::_1( 0xdead ));
  ```

- Some 2-variants `EnumX` that is convertable to 3-variants `EnumX`.

  ```rust
  #[enumx]
  fn f3( variant_index: usize ) -> Enum3<String,i32,bool> {
      match variant_index {
          0 | 1 => f2(variant_index).into(),
          2 => false.into(),
          _ => panic!( "variant index out of bounds" ),
      }
  }
  assert_eq!( f3(0), Enum3::_0( "runtime error".to_string() ));
  assert_eq!( f3(1), Enum3::_1( 0xdead ));
  assert_eq!( f3(2), Enum3::_2( false ));
  ```

- **Variants' order does not matter**.

  ```rust
  #[enumx]
  fn g3( variant_index: usize ) -> Enum3<i32,bool,String> {
      match variant_index {
          0|1 => f2( variant_index ).into(),
            _ => f3( variant_index ).into(),
      }
  }
  assert_eq!( g3(0), Enum3::_2( "runtime error".to_string() ));
  assert_eq!( g3(1), Enum3::_0( 0xdead ));
  assert_eq!( g3(2), Enum3::_1( false ));
  ```

- Those who are not interested in anonymous enum can work with the libs that using anonymouse enum.

  ```rust
  fn h( variant_index: usize ) -> String {
      match f3( variant_index ) {
          Enum3::_0( string ) => string.into(),
          Enum3::_1( errno  ) => format!( "errno: 0x{:x}", errno ),
          Enum3::_2( flag   ) => format!( "flag: {}", flag ),
      }
  }
  assert_eq!( h(0), "runtime error".to_string() );
  assert_eq!( h(1), "errno: 0xdead".to_string() );
  assert_eq!( h(2), "flag: false".to_string() );
  ```

# Limitation

- The library users should use concrete types as the type parameters of EnumX, rather than generics.

- **DO NOT use nested `EnumX` and expect it be flatterned**.

  Currently this project does nothing to support flatterning.

- Current version supports up to 32 variants in a `EnumX`.

  In otherwords, `Enum1`..=`Enum32` is available but `Enum33` but the succeeding enums are not.

  If it is proved that more than 32 variants are useful in practice, they may be supported in later version.

  Notice that the compile time will grow in `O( n*n )` `impl`s.

- Some sort of identifiers are reserved for implementation usage.

  `enumx`/`enum_derive` crates reserve identifiers starting with `__EnumX`, `__enumx` and the identifier `LR`.

# License

Licensed under MIT.
