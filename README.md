# esp32-tests

Used to test the esp32-hal.

## Environment Setup
If you're using CLion, then you should do the following to enable IntelliSense.

1. Link the custom rust build into rustup.
```shell
rustup toolchain link xtensa /{PATH_TO_RUST_XTENSA}/build/{TARGET_TRIPLE}/stage2

# Example
rustup toolchain link xtensa ~/Developer/github/MabezDev/rust-xtensa/build/x86_64-apple-darwin/stage2
```
2. Override the rust toolchain in this directory.
```shell
rustup override set xtensa 
```

3. Source the environment variables
```shell
source setenv.sh
```

4. Open CLion in this directory
```shell
clion .
```

## Flash and Run
Use `cargo-espflash` to flash the board.
```
cargo espflash --example <test> <port>
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
