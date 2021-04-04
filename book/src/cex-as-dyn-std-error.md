# Fallback as `Box<dyn std::error::Error>`

Checked exceptions can work with `std::error::Error` objects as long as all the
variants have implemented `std::error::Error`.

## [Example](#example)

In upstream crate that adopts checked exceptions:

```rust,no_run
use enumx::export::*;
use enumx::predefined::*;
use cex::*;

#[derive( Debug )]
pub struct ErrorA( /* omitted */ );
impl_std_error!( ErrorA );

#[derive( Debug )]
pub struct ErrorB( /* omitted */ );
impl_std_error!( ErrorB );

#[cex] pub fn some_cex_function() -> Result!( () throws ErrorA, ErrorB );
```

In downstream crate that do not adopt checked exceptions:

```rust,no_run
fn downstream() -> Result<(), Box<dyn std::error::Error>> {
    some_cex_function()?;
    Ok( function_from_other_crate()? )
}
```
