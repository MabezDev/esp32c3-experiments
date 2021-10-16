# Rust on the esp32c3

**NOTE** highly experimental at the moment, do not expect support.

## Building and flashing

Add target to your toolchain
```bash
rustup target add riscv32imc-unknown-none-elf
```

Build and flash the image using [`espflash`](https://github.com/esp-rs/espflash)

```bash
cargo espflash /dev/ttyUSB0
```
