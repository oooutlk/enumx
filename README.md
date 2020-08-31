Rust's enum lacks of these features:

1. Ad-hoc enums.
To refer an ad-hoc enum via enumerating all its variants. As an analogy, Rust's
tuples are ad-hoc struct types.

2. Implement traits for enums via delegating to variants.

3. Allow functions to return different types which implement a common trait.

4. Enum exchange.
Conversion between enums which share a common set of variant types.

5. Checked exceptions.
Enumerating all possible error types in function signature, or hiding them via
`Result<_, impl Trait>`.

Four crates categorized into the fowllowing sub projects:

# EnumX, the enum extension library.

  Type definitions in `enumx` crate and proc-macro derives in `enumx_derive` crate.

  See `enumx/README.md` for more.

# CeX, for Checked EXception.

  Type definitions in `cex` crate and proc-macro derives in `cex_derive` crate.

  See `cex/README.md` for more.

# License

Licensed under MIT.
