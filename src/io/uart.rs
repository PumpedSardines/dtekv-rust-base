const JTAG_UART: *mut u32 = 0x04000040 as *mut u32;
const JTAG_CTRL: *mut u32 = 0x04000044 as *mut u32;

pub fn printc(c: u8) {
    unsafe {
        while ((*JTAG_CTRL) & 0xffff0000) == 0 {}
        *JTAG_UART = c as u32;
    }
}

pub fn print(s: &str) {
    for c in s.bytes() {
        printc(c);
    }
}

pub fn println(s: &str) {
    print(s);
    printc(b'\n');
}
