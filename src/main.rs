#![no_main]
#![no_std]

use core::arch::global_asm;

mod lang_items;

global_asm!(include_str!("entry.asm"));