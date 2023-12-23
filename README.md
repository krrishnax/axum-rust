# axum-rust

- To excute and run backend server 
```sh
cargo watch -q -c -w src/ -x run 
```

- Open another terminal to test client side
```sh
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

- [x] [query parameter](https://github.com/krrishnax/axum-rust/pull/1)
- [x] [path parameter](https://github.com/krrishnax/axum-rust/pull/2)