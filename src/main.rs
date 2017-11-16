#![feature(used)]
#![no_std]

extern crate cortex_m_semihosting;
extern crate stm32f40x;


use core::fmt::Write;
use stm32f40x::DWT;
use cortex_m_semihosting::hio;


static mut SEED: u32 = 0x0e0657c1; // The seed to be used to decode the message
static mut PLAIN:[u8; 4] = [0; 4]; // array where decoded chars will be stored
static mut _ABC:[u32; 4] = [0x9fdd9158, 0x85715808, 0xac73323a, 0];  // to test decoding, a known array is given containing a,b,c


fn main() {
    let mut stdout = hio::hstdout().unwrap();
    unsafe {
		(*DWT.get()).enable_cycle_counter();
        (*DWT.get()).cyccnt.write(0);
		let cycles_before = core::ptr::read_volatile(0xE0001004 as *mut u32);
        decode(&mut _ABC, &mut PLAIN);      // call decode with the coded array, the array in which to put the decoded characters and the starting seed
		let cycles_after = core::ptr::read_volatile(0xE0001004 as *mut u32);
        for x in PLAIN.iter()
        {
            write!(stdout, "{}", *x as char).unwrap();
        }
		writeln!(stdout, "{} cycles before, {} cycles after", cycles_before, cycles_after).unwrap();
    }
}

fn codgen() -> u32
{
    let n: i32;
    let x: u32;
    let y: u32;
    unsafe {
    n = SEED.count_zeros() as i32;      // count zeros in seed
    x = SEED.rotate_left(30);           // rotate left 30 bits   
    y = (SEED as i32 >> 6) as u32;     // to get arithmetic shift, set seed as a i32 and then shift to right.
    SEED = x ^ y ^ n as u32;           // set new seed as x xor y xor n
    SEED ^ 0x464b713e                  // return new seed xor 0x464b713e
    }
}

fn decode(word_arr: &mut [u32], byte_arr: &mut [u8]) -> u32
{
    let m: u32;
    let mut r: u32;
    let x: u32;
    let y: u32;

    x = !codgen();      // one's compliment of codgen()
    if word_arr[0] == 0     // last row in word_arr is set to 0 as a breakpoint.
    {
        byte_arr[0] = 0;
        r = x;
    }
    else {
        y = decode(&mut word_arr[1..], &mut byte_arr[1..]);
        m = (x.wrapping_sub(y)).wrapping_sub(word_arr[0]);              // (x-y)- the word_arr_slize
        byte_arr[0]= (m >> 13) as u8;                           // set the byte_arr_slize to the bits at position 20-13 of m.
        r = !codgen() + 0x1;                                            // set r to two's compliment of codgen()
        r = (((x.wrapping_add(y)).wrapping_add(m)).wrapping_add(r)).wrapping_add(5);    // set r to x+y+m+r+5
    }
    r
}
