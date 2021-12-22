# rust-web-server-wasm

rust wasm example

---

- wasmer installation
  
  > wasm runtime
  
  ```shell
  curl https://get.wasmer.io -sSfL | sh
  ```

- add wasm toolchain
  
  > compile rust source to wasm
  
  ```shell
  rustup target add wasm32-unknown-unknown
  ```

- rust compile release
  
  ```shell
  cargo build --target wasm32-unknown-unknown --release
  ```

- wasm-gc installation
  
  > reduce wasm file size
  
  ```shell
  cargo install wasm-gc
  ```

- reduce wasm
  
  ```shell
  wasm-gc target/wasm32-unknown-unknown/release/rust-web-server-wasm.wasm
  ```

- run wasm
  
  ```shell
  cd target/wasm32-unknown-unknown/release/
  wasmer rust-web-server-wasm.wasm -i sum 1 1
  
  # output
  # 2
  ```