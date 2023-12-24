# axum-rust

- To excute and run backend server 
```sh
cargo watch -q -c -w src/ -x run 
```

- Open another terminal to test client side
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
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