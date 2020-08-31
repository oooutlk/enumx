# Ad-hoc enum types

A Rust tuple could be considered as an ad-hoc struct for which the programmers
do not need to name the type nor the fields. As an analogy, an ad-hoc enum is
implicitly defined by its variants.

Unfortunately Rust does not support ad-hoc enums. This library uses `Enum!()`
macros for simulation. For instance, the definition of `Enum!(A,B,C)` is as
follows:

```rust,no_run
enum Enum3<A,B,C> {
    _0( A ),
    _1( B ),
    _2( C ),
}
```

The `Enum!()` macro denotes a series of generic enums named `Enum0`,
`Enum1`, `Enum2`, .. which composed of 0,1,2.. variants. These enums should be
defined beforehand, either predefined in this library, or defined by the library
users.
