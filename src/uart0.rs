//! UART0 panic handler.
//!
//! Writes panic message to UART0 DATA (16C550, offset 0x04) through the
//! flashboot-configured TCXO console (115200 8N1, FIFO on, DLAB=0).
//! Does NOT reconfigure UART0 — if the earlier boot stage did not set it
//! up, this handler will silently spin on `tx_fifo_full`.

use core::panic::PanicInfo;

/// Write bytes to UART0 TX, blocking on FIFO not full.
/// UART0 base = 0x4401_0000, DATA = +0x04, FIFO_STATUS = +0x44.
pub fn write_uart0(msg: &[u8]) {
    // The WS63 UART register block is 32-bit wide even though many fields use
    // only the low 8 or 16 bits.
    const DATA: *mut u32 = 0x4401_0004 as *mut u32;
    const ST: *const u32 = 0x4401_0044 as *const u32;
    for &b in msg {
        unsafe {
            while core::ptr::read_volatile(ST) & 0x01 != 0 {
                core::hint::spin_loop();
            }
            core::ptr::write_volatile(DATA, u32::from(b));
        }
    }
}

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    write_uart0(b"\r\n[PANIC] ");

    // PanicMessage implements Display — format it directly.
    let msg = info.message();
    let mut w = UartWriter;
    let _ = core::fmt::write(&mut w, format_args!("{msg}"));

    if let Some(loc) = info.location() {
        let _ = core::fmt::write(&mut w, format_args!(" @ {}:{}", loc.file(), loc.line()));
    }
    write_uart0(b"\r\n");

    for _ in 0..10_000_000 {
        core::hint::spin_loop();
    }
    loop {
        core::hint::spin_loop();
    }
}

struct UartWriter;

impl core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_uart0(s.as_bytes());
        Ok(())
    }
}
