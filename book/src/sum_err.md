# Support of `?`

This library introduce a proc-macro attribute named `#[sum_err]`, to translate 
the `expr?` expressions in a different manner than the Rust's default:

```rust,no_run
match expr {
    Ok( value ) => value,
    Err( error ) => return Err( #[variant] error ),
}
```

A `#[sum]` tagged function should be tagged with `#[sum_err]` if it contains `?`
expressions.

## Example

```rust,no_run
#[sum_err]
#[sum( impl Clone )]
fn foo( branch: i32 ) -> Result<(), impl Clone> {
    match branch % 3 {
        0 => Ok(()),
        1 => Ok( Err( 0 )? ),
        2 => Ok( Err( "lorum" )? ),
        _ => unreachable!(),
    }
}
```

Note: put `#[sum_err]` **before** `#[sum]`.
