# Fallback as `impl Trait`

Sometimes the downstream users do not bother "poluting" signatures of functions
which call upstream `#[cex]` APIs returning checked exceptions. If all variants
of the error `Enum!()` have implemented some famous trait, e.g.
`std::error::Error`, the downstream users get a chance to simply write
`-> Result<_, impl std::error::Error>` in their function signatures.

## [Example](#example)

```rust,no_run
use std::error::Error;
use enumx::export::*;
use enumx::predefined::*;
use cex::*;

impl Error for A { /* omitted */ }
impl Error for B { /* omitted */ }
impl Error for C { /* omitted */ }

#[cex] pub fn some_cex_function() -> Result!( () throws A, B, C );

fn downstream() -> Result<(), impl Error> {
    Ok( some_cex_function()? )
}
```
