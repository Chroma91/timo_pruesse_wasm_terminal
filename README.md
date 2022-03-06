# About timo_pruesse_wasm_terminal

A "terminal interpreter" for the commands that you can run on [timo-pruesse.de](https://timo-pruesse.de/).

This project is purely made for educational purposes and to experiment with `Rust` and `WASM` in the browser 😊.

-----

## Build

You need `wasm-pack` to build it for the web.
It can be installed with cargo: `cargo install wasm-pack`.

```bash
wasm-pack build --target=web
```

## Publish

```bash
wasm-pack publish
```
