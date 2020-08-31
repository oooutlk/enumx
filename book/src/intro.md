# Introduction to enumx

This project provides ENUM eXtensions to simulate the following features:

- "Union types" in Racket, with the special interest in error-handling, aka
checked exception.

- summaries into an enum, the returned values of different types by functions
that return `impl Trait`.

- macros to help implementing traits for enums the variants of which have all
implemented the traits.

Four crates categorized into the fowllowing sub projects:

## EnumX, the enum extension library.

Type/trait definitions in `enumx` crate and proc macros in `enumx_derive` crate.

## CeX, for Checked EXception.

Type/trait definitions in `cex` crate and proc macros in `cex_derive` crate.
