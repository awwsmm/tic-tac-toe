# Tic Tac Toe

## Compiling From Source

### Prerequisites

Install `rustc`, `rustup`, and `cargo` [by running the following command](https://rustup.rs/) in a terminal

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once `rustup` is properly installed, add the `wasm32-unknown-unknown` compilation target

```shell
rustup target add wasm32-unknown-unknown
```

Once `cargo` is properly installed, use it to install the [`wasm-bindgen-cli`](https://github.com/rustwasm/wasm-bindgen)

```shell
cargo install wasm-bindgen-cli
```

...and [`live-server`](https://github.com/lomirus/live-server)

```shell
cargo install live-server
```

### Compiling

Compile the Rust code to WASM with

```shell
cargo build --release --target wasm32-unknown-unknown
```

Generate JavaScript bindings for the WASM output with

```shell
wasm-bindgen --out-dir target --target web target/wasm32-unknown-unknown/release/tic-tac-toe.wasm
```

### (Optional) Automatically Recompile

If you want `cargo` to automatically recompile the Rust code in this repo when it changes, install [`cargo-watch`](https://github.com/watchexec/cargo-watch) with

```shell
cargo install cargo-watch
```

...and then set up `cargo watch` to run the steps in the "Compiling" section, above, automatically

```shell
cargo watch -- bash build-and-bind.sh
```

Note that this will block a terminal, so all other commands will need to be run in a separate terminal window.

## Running

### Locally

To view the compiled output locally, execute the following command in a terminal

```shell
live-server
```

This should give output like

```
[2024-02-26T13:38:10Z INFO  live_server::server] Listening on http://192.168.2.24:52746/
[2024-02-26T13:38:10Z INFO  live_server::watcher] Listening on /Users/andrew/Git/tic-tac-toe
```

You can then view the output by opening the specified address (e.g. `http://192.168.2.24:52746/`) in a browser.

If you followed the steps in the "Automatically Recompile" section, above, this page will be automatically refreshed as changes are made to the code in this repository.

Press Control-C in the terminal to close the HTTP server.