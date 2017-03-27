#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
extern crate rlibc;
extern crate volatile;

mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::print_something();
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
