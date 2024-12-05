//! Building this module successfully guarantees that the library is no-std compatible

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use staged_sg_filter;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // We can't print, so there's not much to do here
    loop {}
}

#[no_mangle]
pub fn _start() -> ! {
    const N: usize =  10;
    let v = [10.0; N];
    let mut buf = [0.0; N];
    staged_sg_filter::sav_gol::<2, 2>(&mut buf, &v);
    let _ = buf[0];

    loop {} // We don't actually run this, just compile it
}
