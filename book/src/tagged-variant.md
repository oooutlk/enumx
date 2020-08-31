# The variant attribute

The `#[sum]` tag collects all expressions with `#[variant]` attributes and wraps them with enum constructors in the form of `SomeEnumName::_0`,
`SomeEnumName::_1`.. respectively.  For example, the function body in previous
[example](./sum.md#sum-example) will be expanded to:

```rust,no_run
if cond {
    __SumType2::_0( 1_i32 )
} else {
    __SumType2::_1( "false" )
}
```

## Merge variants of the same types

The `#[variant]` attribute supports merging by giving the same name of merged
variants. For example, a series of expresions with `#[variant( foo )]`,
`#[variant]`, `#[variant( foo )]` will be wrapped with `_0`, `_1`, `_0`.
