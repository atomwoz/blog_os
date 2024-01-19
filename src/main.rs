// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::{panic::PanicInfo, usize};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn wait(time: i32)
{
    let mut i = 0;
    while i < time {
        let mut j=0;
        while j<3_000_000
        {
            j+=1
        }
        i+=1;
    }
}

static HELLO: &[u8] = b"Exec-it V8 is starting, brum, brum... Lorem ipsum lorem ipsum Exec-it V8 is starting, brum, brum... Lorem ipsum lorem ipsum Exec-it V8 is starting, brum, brum... Lorem ipsum lorem ipsum";
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const CONSOLE_WIDTH: usize = 160;
const CONSOLE_ROWS: usize = 25;

fn generate_color_pallette(mut pallete: &[u32])
{
    
}

fn println(data: &str, line: usize)
{
    let start_addr = VGA_BUFFER.wrapping_add(line*CONSOLE_WIDTH);
    for (i, &byte) in data.as_bytes().iter().enumerate() {
        unsafe {
            *start_addr.offset(i as isize * 2) = byte;
            *start_addr.offset(i as isize * 2 + 1) = FONT_STYLE;
            wait(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println("The butterfly effect", 1);
    println("Yeah im making her mad", 2);
    println("She always say that I'm the best", 3);
    loop {}
}