# The Exchange trait

This library provides `ExchangeFrom`/`ExchangeInto` traits which is similar to
std `From`/`Into` but with extra phantom generic type.

```rust,no_run
pub trait ExchangeFrom<Src, Index> {
    fn exchange_from( src: Src ) -> Self;
}

pub trait ExchangeInto<Dest, Index> {
    fn exchange_into( self ) -> Dest;
}

```

Blanket implementations of `ExchangeInto` are similar to `Into`:

```rust,no_run
impl<Src, Dest, Index> ExchangeInto<Dest, Index> for Src
    where Dest: ExchangeFrom<Src, Index>,
{
    fn exchange_into( self ) -> Dest {
        Dest::exchange_from( self )
    }
}
```

Any enum in the form described below can derive these traits automatically, by
using `#[derive( Exchange )]`.

```rust,no_run
use enumx::export::*;

#[derive( Exchange )]
enum Data {
    Bin( Vec<u8> ),
    Text( String ),
}

#[derive( Exchange )]
enum Value {
    Bin( Vec<u8> ),
    Text( String ),
    Literial( &'static str ),
}

// use ExchangeFrom
let data = Data::exchange_from( "foo".to_owned() );
let value = Value::exchange_from( data );

// use ExchangeInto
let data: Data = "foo".to_owned().exchange_into();
let value: Value = data.exchange_into();
```

This library provides predefined enums that have implement
`ExchangeFrom`/`ExchangeInto`. The user can `use enumx::predefined::*;`, and use
`Enum!()` macro to denote types, as described in
[Ad-hoc enum types](./ad-hoc-enums.md).

Alternatively, the user is able to define their own ad-hoc enum types:

```rust,no_run
use enumx::export::*;
// do not use enumx::predefined::*;

def_impls! {
    #[derive( Exchange )]
    pub enum Enum![ 1..=16 ];
}
```
