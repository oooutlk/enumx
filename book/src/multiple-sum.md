# Multiple sum tags

This library supports to tag a function with multiple `#[sum]` attributes,
which summarize different `impl Trait` into different enums.

## More syntax of `#[sum]` and `#[variant]`

1. `#[sum( sum_name => impl Trait )]`

2. `#[sum( sum_name => impl Trait for Enum )]`

3. `#[variant( sum_name => variant_name )]`

4. `#[variant( sum_name => _ )]`

The `sum_name` tells which `impl Trait` enum the `#[sum]`/`#[variant]` belongs
to.

```rust,no_run
#[sum( ok  => impl Clone )]
#[sum( err => impl Clone )]
fn sum_okeys_and_errors( branch: i32 ) -> Result<impl Clone, impl Clone> {
    match branch % 4 {
        0 => Ok(  #[variant( ok  => _ )] branch ),
        1 => Ok(  #[variant( ok  => _ )] () ),
        2 => Err( #[variant( err => _ )] branch ),
        3 => Err( #[variant( err => _ )] () ),
        _ => unreachable!(),
    }
}
```
