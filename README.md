# Static File Server

This is a simple [axum](https://github.com/tokio-rs/axum) based static file server.

Build and use `static-file-server --dir <> --port <>`.

## Cross Compile

* `cargo build --release --target "x86_64-unknown-linux-musl"`