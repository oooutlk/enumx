# The contextual macros

This library introduces a proc macro `def_impls!{}` in which you can define an
enum and write several impl blocks. In these impl blocks, the following macros
are supported:

1. `_Variants!()` in where clause.
For example, `where _Variants!(): SomeTrait` means each variant has the trait
bound of `: SomeTrait`.

2. `_match!()` in trait method.
This macro will expand to a match expression that enumerate all variants, and
translate the macros listed below.

3. `_variant!()` in `_match!()`.
This macro will expand to the value of the matched variant.

4. `_Variant!()` in `_match!()`.
This macro will expand to the type of the matched variant.

5. `_enum!()` in `_match!()`.
This macro will wrap its inner value to get an enum. Use this macro if the trait method returns `Self`. 

## Examples

Let's rewrite the [generic enum example](./impl-traits-for-enums.md#generic-enum-example):

```rust,no_run
def_impls! {
    impl<T0,T1> AsRef<[u8]> for Enum2<T0,T1>
        where _Variants!(): AsRef<[u8]> 
    {
        fn as_ref( &self ) -> &[u8] {
            _match!(
                _variant!().as_ref()
            )
        }
    }
}
```

Another example, implementing Clone. Note the using of `_enum!()`.

```rust,no_run
def_impls! {
    impl<T0,T1> Clone for Enum2<T0,T1>
        where _Variants!(): Clone
    {
        fn clone( &self ) -> Self {
            _match!(
                _enum!( _variant!().clone() )
            )
        }
    }
}
```

You can specify the expression `matched` being matched, using the syntax
`_match!( matched => expr )`. If ommited, `_match!( expr )` is equivalent to
`_match!( self => expr )`.

```rust,no_run
def_impls! {
    impl<T0, T1, R, Yield, Return> std::ops::Generator<R> for Enum2<T0,T1>
        where _Variants!(): std::ops::Generator<R,Yield=Yield,Return=Return>
    {
        type Yield = Yield;
        type Return = Return;
        fn resume( self: std::pin::Pin<&mut Self>, arg: R )
            -> std::ops::GeneratorState<Self::Yield, Self::Return>
        {
            _match!( unsafe{ self.get_unchecked_mut() } =>
                 unsafe{ std::pin::Pin::new_unchecked( _variant!() )}
                    .resume( arg )
            )
        }
    }
}
```
