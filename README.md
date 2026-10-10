# ModDevGenerator

Website that generates Minecraft mod development templates.

## Run

```sh
devenv shell
dev     # dx serve --platform web -> http://localhost:8080
build   # release build -> target/dx/moddev-generator/release/web/public
```

Plain cargo works inside the shell as well:

```sh
cargo check --target wasm32-unknown-unknown
cargo clippy --all-targets --target wasm32-unknown-unknown
cargo test
cargo fmt
```

`wasm-bindgen` is pinned in `Cargo.toml`, because dx refuses to build unless it matches the
`wasm-bindgen-cli` that ships with `dioxus-cli`.
