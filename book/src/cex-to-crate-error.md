# Fallback as "crate error"

The checked exceptions are not necessarily infectious. The library author could
provide the crate error type for library users to use in their own functions the
error types of which are all from the library.

## [Example](#example)

In upstream crate that adopts checked exceptions:

```rust,no_run
use enumx::export::*;
use enumx::predefined::*;
use cex::*;

#[derive( Debug )] // not mandatory
pub struct ErrorA( /* omitted */ );

#[derive( Debug )] // not mandatory
pub struct ErrorB( /* omitted */ );

#[derive( Debug )] // not mandatory
pub struct ErrorC( /* omitted */ );

crate_error!{
    #[derive( Debug )] // not mandatory
    pub enum CrateError {
        ErrorA,
        ErrorB,
        ErrorC,
    }
}

#[doc( hidden )]
pub trait IntoCrateError {
    fn into_crate_error( self ) -> CrateError;
}

impl<E: IntoCrateError> From<E> for CrateError {
    fn from( e: E ) -> Self { e.into_crate_error() }
}

def_impls! {
    // the maximium variant count is 4 in upstream crate.
    impl IntoCrateError for Enum![1..=4]
        where _Variants!(): Into<CrateError>
    {
        fn into_crate_error( self ) -> CrateError {
            _match!(
                _variant!().into()
            )
        }
    }
}

pub type CrateResult<T> = Result<T, CrateError>;

#[cex] pub fn some_cex_function() -> Result!( () throws ErrorA, ErrorB );
#[cex] pub fn another_cex_function() -> Result!( () throws ErrorA, ErrorC );

```

In downstream crate that do not adopt checked exceptions:

```rust,no_run
fn downstream() -> upstream::CrateResult<()> {
    some_cex_function()?;
    Ok( another_cex_function()? )
}
```
