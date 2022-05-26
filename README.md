# r2d2-oracle
[![Build Status](https://travis-ci.com/rursprung/r2d2-oracle.svg?branch=master)](https://travis-ci.com/rursprung/r2d2-oracle)
[![Crates.io](https://img.shields.io/crates/v/r2d2-oracle)](https://crates.io/crates/r2d2-oracle)
![Crates.io](https://img.shields.io/crates/l/r2d2-oracle)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

The documentation can be found on [docs.rs](https://docs.rs/r2d2-oracle/).

Oracle support for the r2d2 connection pool.
This fits in between the [r2d2](https://crates.io/crates/r2d2) connection manager and [oracle](https://crates.io/crates/oracle) database driver crates.

## Usage
See the documentation of r2d2 for the details on how to use the connection pool.

```rust
use std::thread;
use r2d2_oracle::OracleConnectionManager;

fn main() {
    let manager = OracleConnectionManager::new("user", "password", "localhost");
    let pool = r2d2::Pool::builder()
         .max_size(15)
         .build(manager)
         .unwrap();
    
    for _ in 0..20 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            // use the connection
            // it will be returned to the pool when it falls out of scope.
        });
    }
}
```

If you want to use chrono data types, enable the `chrono` feature:

```toml
[dependencies]
r2d2-oracle = { version = "0.2.0", features = ["chrono"] }
```

## Changelog
For the changelog please see the dedicated [CHANGELOG.md](CHANGELOG.md).

## Current Status of the Crate & Roadmap to v1.0.0
This is the initial release of the crate and has not yet been proven in production. Nevertheless: the crate is very small so not many problems are expected.
The precondition for releasing v1.0.0 is that both `r2d2` and `oracle` have released their v1.0.0.

## Alternatives to `r2d2-oracle`
You may also want to consider the following alternatives to this crate, depending on your use-cases:
* Starting with version 0.5.5 the [`oracle`](https://crates.io/crates/oracle) provides buit-in connection pooling support
* There is an `async` ([`tokio`](https://crates.io/crates/tokio)-based) version of `r2d2`, [`bb8`](https://crates.io/crates/bb8) and a corresponding [`bb8-oracle`](https://crates.io/crates/bb8-oracle) fork of `r2d2-oracle` exists

## Build-time Requirements
The crate is tested against stable rust and rust 1.54.0 (which was the stable version at the time the crate has been built).
It is possible that it works with older versions as well but this is not tested.
Please see the details of the r2d2 and oracle crates about their requirements.
