#![no_std]
#![no_main]
mod vga_buffer;

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Romeo: Yo, Mercutio, my man! Why the long face?\nMercutio: Romeo, \
    my brother from another mother, I'm just feeling some bad vibes today, you know? \
    Like the universe is messing with my flow.");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
