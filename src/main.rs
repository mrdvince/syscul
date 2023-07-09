#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "Romeo: Yo, Mercutio, my man! Why the long face?\nMercutio: Romeo, \
    my brother from another mother, I'm just feeling some bad vibes today, you know? \
    Like the universe is messing with my flow."
    );

    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion.....");
    assert_eq!(1,2);
    println!("[ok]")
}