# `decode_embedded`

> An implementation of a decoding assignment implemented on a Nucleo 64 using Rust.

# Branches

A. Backport assignment 3b to your chosen target. Use semihosting in order to write the resulting string to the host. You may need to use --release for decoding the long (coded) message, as being deeply recursive unoptimized code may run out of stack memory.

B. Discuss from a memory safety perspective the outcome.

C. Compare for the short message (abc), the number of cycles required for decode in debug (standard) vs. --release. As a comparison my straightforword C implementation took 2200 cycles in best optimized mode using gcc (-o3), while my (translation) to Rust code took 1780 cycles (--release). (Both executed on a bluepill board at 8MHz without (flash) memory wait states).

