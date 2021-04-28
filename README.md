# Rust on the esp32c3

**NOTE** highly experimental with no emphasis on usability, do not expect support.

## Building and flashing

As the target is builtin to rust already, simply run 
```bash
cargo build --target riscv32imc-unknown-none-elf
```

Convert elf to esp image
```bash
esptool.py --chip esp32c3 elf2image --flash_mode=dio -o esp32c3.bin target/riscv32imc-unknown-none-elf/debug/esp32c3
```

Flash image

```bash
esptool.py --chip esp32c3 -p /dev/ttyUSB0 --after hard_reset write_flash 0x0 esp32c3.bin
```