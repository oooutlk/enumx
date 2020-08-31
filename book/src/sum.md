# Sum `impl Trait`

This is an extension to allow multiple return types which implement the same trait.

## [Example](sum-example)

```rust,no_run
#[sum]
fn f( cond: bool ) -> impl Clone {
    if cond {
        #[variant] 1_i32
    } else {
        #[variant] "false"
    }
}
```
