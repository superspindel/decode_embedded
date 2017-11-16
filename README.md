# `decode_embedded C`

> Implementation of decode assignment on a Nucleo64 using Rust.

# How to build

1. `git clone https://github.com/superspindel/decode_embedded.git`
2. `cd decode_embedded`
3. `TARGET=thumbv7em-none-eabihf`
4. `xargo build --target $TARGET --release`

# How to run
5. Open up two terminal windows in source path.
6. `openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg` on terminal window 1.
7. Switch to terminal window 2.
7. `echo 'add-auto-load-safe-path /' >> ~/.gdbinit`
8. `arm-none-eabi-gdb target/$TARGET/release/decode_embedded`
9. `continue`
10. Switch to terminal window 2.
11. Read output decoded string.


## Number of cycles
### Release
> 328 cycles
### Debug
> 4260 cycles
