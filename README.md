# secrust-santa

Secret Santa implemented in Rust 🦀

## Back

- Using [Rocket](https://rocket.rs/)
- Persistence: Postgres

### Development Environment

`secrust-santa` is based on Diesel to interact with the PostgreSQL DB.
To use it, you can install the Diesel CLI :
```Rust
cargo install diesel_cli --no-default-features --features postgres
```

### Data model

- Session
  - Identifier: uuid
  - Name

- Participant
  - Identifier: uuid
  - Name
  - Linked to a session

- Blacklist:
  - List of uuid tuples
  - Linked to a session

## Front

- Web Assemby with [yew](https://yew.rs/)

