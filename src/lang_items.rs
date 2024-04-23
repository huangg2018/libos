use core::panic::PanicInfo;
use crate::console::*;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!()
    }
}