# Predefined ad-hoc enums

This library has defined `Enum0`, `Enum1` .. up to `Enum16` by default.

The library user can `use enumx::predefined::*;` for convenience.

A feature named "enum32" increases the set of predefined enums up to `Enum32`.

`Cargo.toml`:

```toml
[dependencies.enumx]
version = "0.4"
features = "enum32"
```

The predefined enums can be disabled by opting out "Enum16" and "Enum32" features.

`Cargo.toml`:

```toml
[dependencies.enumx]
version = "0.4"
default-features = false
```
