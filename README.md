# sea-orm-typed-id

[![Crates.io](https://img.shields.io/crates/v/sea-orm-typed-id.svg)](https://crates.io/crates/sea-orm-typed-id)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library that provides a macro for generating type-safe database ID types for SeaORM.

## Caveat

You might actually not need this library, it's just a macro and you might be better off with just copying the code from [src/lib.rs](src/lib.rs) into your project.

### Features

- `all`: Enables all features
- `serde`: Enables serde support for serialization/deserialization
- `rustls`: Enables rustls TLS backend for SeaORM
- `postgres`: Enables PostgreSQL array support
- `utoipa`: Enables OpenAPI schema generation support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sea-orm-typed-id = { version = "0.1.0", features = ["all"] }
```

## Usage

```rust
use sea_orm_typed_id::define_id;

define_id!(UserId);

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "cakes")]
pub struct Cake {
    pub id: CakeId,
}

define_id!(FillingId);

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "fillings")]
pub struct Filling {
    pub id: FillingId,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "cake_fillings")]
pub struct CakeFilling {
    pub cake_id: CakeId,
    pub filling_id: FillingId,
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
