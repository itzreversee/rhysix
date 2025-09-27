## Web build
If you wish to build for web, here are the instructions (tested for linux).

- Install [emscripten](https://emscripten.org/docs/getting_started/downloads.html#), for Arch, you can use `pacman -S emscripten`
- Add the `wasm32-unknown-emscripten` target using rustup: `rustup target add wasm32-unknown-emscripten`
- Build the game using this command (i'll make it into a script one day i promise):
```
EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1 -sEXPORTED_RUNTIME_METHODS=requestFullscreen" cargo build --release --target=wasm32-unknown-emscripten
```
- Copy the runner from `docs/emscripten_runner.html` to `target/wasm32-unknown-emscripten/release/index.html`
- Copy the output files `index.html`, `rhysix.js` and `rhysix.wasm` to some silly location.
- Serve using your http server, e.g. python: `python -m http.server -d .`