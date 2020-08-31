# User defined ad-hoc enums

Sometimes the library users have to define these ad-hoc enums themselves to
implement more traits which are not implemented for predefined enums in this
library.

```rust,no_run
use enumx::export::*;

def_impls! {
    #[derive( SomeTraitNeverHeardByEnumxAuthor )]
    enum Enum![ 0..=16 ];
}
```

`Enum![ 0..=16 ]` means `Enum0`,`Enum1`,.. up to `Enum16`. The name `Enum` can
be replaced by any valid identity. For instance, `MyEnum![ 1..=6 ]` means
`MyEnum1`, `MyEnum2`, up to `MyEnum6`.

Where clause is supported by `def_impls!{}`.

```rust,no_run
    use enumx::export::*;

    def_impls! {
        pub enum Enum![ 0..=16 ]
            where _Variants!(): Iterator<Item=i32>;
    }
```
