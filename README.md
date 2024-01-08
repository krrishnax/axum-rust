# axum-rust

- To excute and run backend server 
```sh
cargo watch -q -c -w src/ -x run 
```

- Open another terminal to test client side
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
- Note: If you haven't already, make sure to install Cargo Watch.
```sh
cargo install cargo-watch
```

## axum steps:
- [x] 1 [query parameter](https://github.com/krrishnax/axum-rust/pull/1)
- [x] 2 [path parameter](https://github.com/krrishnax/axum-rust/pull/2)
- [x] 3 [static routes](https://github.com/krrishnax/axum-rust/pull/3)
- [x] 4 [first login api](https://github.com/krrishnax/axum-rust/pull/4)
- [x] 5 [login auth-token cookies](https://github.com/krrishnax/axum-rust/pull/5)
- [x] 6 [REST api - Mock model/store](https://github.com/krrishnax/axum-rust/pull/6)
- [x] 7 [CRUD - REST Api](https://github.com/krrishnax/axum-rust/pull/7)
- [x] 8 [middleware](https://github.com/krrishnax/axum-rust/pull/8)
- [x] 9 [token parsing](https://github.com/krrishnax/axum-rust/pull/9)
- [x] 10 [Ctx Extractor - using async trait](https://github.com/krrishnax/axum-rust/pull/10)
- [x] 11 [Ctx in REST API](https://github.com/krrishnax/axum-rust/pull/11)
- [x] 12 [Ctx Resolver - for optimization](https://github.com/krrishnax/axum-rust/pull/12)
- [x] 13 [Errors - for client and server](https://github.com/krrishnax/axum-rust/pull/13)
- [x] 14 [Server Log line](https://github.com/krrishnax/axum-rust/pull/14)
