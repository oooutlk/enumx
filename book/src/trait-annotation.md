# Trait annotation

The `#[sum]` tag will analyze the function's return type and decide which
`impl Trait` to summarize. If it is not what you want, use
`#[sum( impl Trait )]` to annotates the `impl Trait` explicitly.

If both trait annotation and externally defined enum type are required, use
`#[sum( impl Trait for Enum )]`.



