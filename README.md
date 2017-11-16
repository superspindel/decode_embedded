# `decode_embedded B`

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


## Discussion from memory safety perspective

There isn't really any changes when going to embedded from a memory safety perspective. We still have unsafe implementations since there are STATIC variables, and the outcome is the same as before. This is good since that means that there are nothing hindering you from implementing code on embedded platforms using rust and the compiler can still ensure memory safety on these devices.
