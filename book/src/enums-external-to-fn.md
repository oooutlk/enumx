# Enums external to function

In previous [example](./sum.md#sum-example), the `#[sum]` tag will generate an
enum type local to the function. An anternative way is to use externally
defined enums, for which the library users can implement traits manually.

```rust,no_run
use serde::{Serialize, Serializer};

use enumx::export::*;
// DO NOT use enumx::predefined::*;

def_impls! {
    enum Enum![2..=3];

    impl Serialize for Enum![2..=3]
        where _Variants!(): Serialize
    {
        fn serialize<S: Serializer>( &self, serializer: S ) -> Result<S::Ok, S::Error> {
            _match!( _variant!().serialize( serializer ))
        }
    }
}

#[sum( Enum )]
fn f( cond: bool ) -> impl Serialize {
    if cond {
        #[variant] 1_i32
    } else {
        #[variant] "false"
    }
}

#[sum( Enum )]
fn g( cond: u32 ) -> impl Serialize {
    match cond % 3 {
        0 => #[variant] 1_i32,
        1 => #[variant] "false",
        2 => #[variant] true,
        _ => unreachable!(),
    }
}
```

