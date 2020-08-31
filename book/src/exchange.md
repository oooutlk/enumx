# Enum exchange

Suppose `Enum!(A,B,C,..)` denotes a type similar to `enum` that composed of
variants A,B,C.., with extra features:

1. Variants of the duplicated type are merged.
  For instance, `Enum!(A,B,A)`  is `Enum!(A,B)`.

2. Order of variants does not matter.
  For instance, `Enum!(A,B)` is `Enum!(B,A)`.

3. `Enum!()`s as variants are flattened.
  For instance, `Enum!(A, Enum!(B,C))` is `Enum!(A,B,C)`.

4. Any subset of an `Enum!()` can be converted to it.
  For instance, `A`, `Enum!(A)` and `Enum!(A,B)` can be converted to
`Enum!(A,B,C)`.

Such types, which are similar to Racket's "union types", do not exist in Rust's
type systems. With the help of this library, the best we can get is
"union values":

1. `Enum!()`s that has duplicated variant types **cannot** be converted to each
other without extra annotation, which is not practicable.

2. Two `Enum!()`s composed of the same variant types but with different order
can be converted to each other.
  For instance, `Enum!(A,B)` can be converted to `Enum!(B,A)`, and vise vesa.

3. `Enum!()`s as variants are **not** flattened in conversion.
  This library might support converting `Enum!(A,C)` to `Enum!(A, Enum!(B,C))` in
the future( perhaps if Rust supports `where T != U` ), but not now.

4. Any subset of an `Enum!()` can be converted to it.

This library names the conversion in #2 and #4 as "enum exchange", and defines
an derivable [`Exchange` trait](./exchange-trait.md).
