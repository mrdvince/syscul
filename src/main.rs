#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

#[allow(unused_imports)]
use core::panic::PanicInfo;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

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

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::{Port, PortGeneric};
    unsafe {
        let mut port: PortGeneric<u32, x86_64::instructions::port::ReadWriteAccess> =
            Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success)
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion.....");
    assert_eq!(1, 2);
    serial_println!("[ok]")
}
