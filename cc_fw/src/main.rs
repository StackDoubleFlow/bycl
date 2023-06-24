#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"), sym entry);

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

fn entry() -> ! {
    let (a, b, out) = unsafe {
        (
            *(0x100 as *const u32),
            *(0x104 as *const u32),
            &mut *(0x108 as *mut u32),
        )
    };
    *out = a % b;
    // do_thing();
    loop {}
}
