# impl traits for enums

It is very common to implement traits for enums if all of its variants have
implemented the traits.

```rust,no_run
enum Data {
    Bin( Vec<u8> ),
    Text( String ),
}

impl AsRef<[u8]> for Data {
    fn as_ref( &self ) -> &[u8] {
        match self {
            Data::Bin(  v ) => v.as_ref(),
            Data::Text( v ) => v.as_ref(),
        }
    }
}
```

[In case of generic enums](#generic-enum-example):

```rust,no_run
impl<T0,T1> AsRef<[u8]> for Enum2<T0,T1>
    where T0: AsRef<[u8]> 
        , T1: AsRef<[u8]> 
{
    fn as_ref( &self ) -> &[u8] {
        match self {
            Enum2::_0( s ) => s.as_ref(),
            Enum2::_1( s ) => s.as_ref(),
        }
    }
}
```

The basic idea is to match the variants and delegate. This library provides
macros to help to avoid writing these boilerplate code:

1. [The contextual macros](./contextual-macros.md)

These macros helps not repeating the where clauses and match arms of each variant.

2. [The predefined macros](./predefined-macros.md)

These macros helps to omit the methods in impl blocks.
