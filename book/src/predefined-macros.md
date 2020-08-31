# The predfined macros

For frequently used traits in std, this library provides macros such as 
`impl_trait!()` to implement these traits without the need of writing trait
methods.

## Syntax of `impl_trait!{}`

The full form is

```rust,no_run
impl_trait! {
    _impl!( Generics ) Path::Of::Trait _for!( Type ) _where!( Clause )
}
```

`Generics` and `Clause` are optional:

```rust,no_run
impl_trait!{ _impl!() Path::Of::Trait _for!( Type ) _where!() }
```
and the wrapped macros can be omitted:

```rust,no_run
impl_trait!{ Path::Of::Trait _for!( Type )}
```

## Supported forms of types in `_for!()`

The `_for!()` macro supports two forms of types.

One is ad-hoc enums:


```rust,no_run
impl_trait!{ Path::Of::Trait _for!( Enum![1..=16] )}
```

The other is the enum type definition copied in `_def!()` macro:


```rust,no_run
impl_trait!{ Path::Of::Trait _for!( _def!(
    enum Value {
        Bin( Vec<u8> ),
        Text( String ),
    }
))}
```

Note: `_def!()` does not define any enum, so `Value` should have been defined elsewhere.

## The `_where!()` macro

You can write any where clause in this macro.

Note: you do not need write `_where!( _Variants!(): Path::Of::Trait )` which the
`impl_trait!{}` macro will generate it silently.

## Traits in std prelude

`AsRef`

`AsMut`

`DoubleEndedIterator`

`ExactSizeIterator`

`Extend`

`Fn`

`Iterator`

The example of implementing `Iterator`:

```rust,no_run
impl_trait!{ Iterator _for!( Type )}
```

The example of implementing `Fn`:

```rust,no_run
impl_trait!{ _impl!(Args) Fn<Args> _for!( Type )}
```

## Traits with full path

`std::error::Error`

`std::fmt::Debug`

`std::fmt::Display`

`std::iter::FusedIterator`

`std::iter::TrustedLen`

`std::io::BufRead`

`std::io::Read`

`std::io::Seek`

`std::io::Write`

`std::ops::Deref`

`std::ops::DerefMut`

`std::ops::Generator`

`std::ops::Index`

`std::ops::IndexMut`

`std::ops::RangeBounds`

The example of implementing `std::ops::Generator`:

```rust,no_run
impl_trait!{ _impl!(R) std::ops::Generator<R> _for!( Type )}
```

## Unstable traits 

To implement these traits, the crate feature "unstable" should be opted in.

`Fn`

`std::iter::TrustedLen`

`std::ops::Generator`

## `impl_super_traits!{}` and `impl_all_traits!{}`

The syntax of these two traits are similar with `impl_trait!{}`.

The `impl_super_traits!{}` macro helps to implement the super trait(s) of the
mentioned trait, e.g. `impl_super_traits!{ _impl!(Args) Fn<Args> _for!( Type )}`
 will implement `FnMut` and `FnOnce` for `Type`, but **NOT** `Fn`.

The `impl_all_traits!{}` macro does what `impl_trait!{}` and
`impl_super_traits!{}` does, e.g.
`impl_all_traits!{ _impl!(Args) Fn<Args> _for!( Type )}` will implement `Fn`,
`FnMut` and `FnOnce` for `Type`.

## macro inheritance

If the library users want to support extra traits, they can write the extra
implementations in their macro, and delegate other traits to
`enumx::impl_trait!()`.

```rust,no_run
use enumx::export::{def_impls, impl_all_traits};

macro_rules! impl_trait {
    ($(_impl!($($gen:tt),*))* ExtraTrait<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        // omitted
    };
    ($($tt:tt)+) => {
        enumx::impl_trait!{ $($tt)+ }
    };
}


macro_rules! impl_super_traits {
    ($(_impl!($($gen:tt),*))* ExtraTrait<$t:ident> _for!($($ty:tt)+) $(_where!($($pred:tt)*))*) => {
        // omitted
    };
    ($($tt:tt)+) => {
        enumx::impl_super_traits!{ $($tt)+ }
    };
}
```
