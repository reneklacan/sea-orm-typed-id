# sea-orm-typed-id

[![Crates.io](https://img.shields.io/crates/v/sea-orm-typed-id.svg)](https://crates.io/crates/sea-orm-typed-id)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library that provides a macro for generating type-safe database ID types for SeaORM.

## Caveat

You might not actually need this library; it’s just a macro, and you might be better off simply copying the code from src/lib.rs into your project.

## Features

- `all`: Enables all features
- `rustls`: Enables rustls TLS backend for SeaORM
- `postgres`: Enables PostgreSQL array support
- `schema`: Enables `schemars::JsonSchema` support
- `utoipa`: Enables OpenAPI schema generation support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sea-orm-typed-id = { version = "0.4.1", features = ["all"] }
```

## Usage

```rust
use sea_orm_typed_id::define_id;

define_id!(CakeId);

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

## Known Shortcomings

Typed IDs won't work with postgres arrays.

```rust
// ...
pub struct Model {
    // ...
    pub filling_ids: Vec<FillingId>, // Doesn't work
}
```

This won't compile because `Vec<FillingId>` doesn't implement [`sea_orm::TryGetable`](https://docs.rs/sea-orm/latest/sea_orm/trait.TryGetable.html) which we also can't add ourselves as both `Vec` and `sea_orm::TryGetable` are external.

One of possible workouts is to make fields private and add getter for it.

```rust
// ...
pub struct Model {
    // ...
    filling_ids: Vec<i32>,
}

impl Model {
    pub fn filling_ids(&self) -> Vec<FillingId> {
        self.filling_ids.iter().map(FillingId::from).collect()
    }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
