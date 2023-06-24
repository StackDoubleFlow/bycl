#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn do_thing() {
    unsafe {
        let ptr = 0x69696969 as *mut u8;
        *ptr = 5;
    }
}

#[no_mangle]
fn cc_fw_entry() -> ! {
    do_thing();
    loop {}
}
