//! Panic handlers for HiSilicon RISC-V chips.
//!
//! # Features
//!
//! | Feature | Behaviour |
//! |---------|-----------|
//! | `panic-halt` (default) | Silent CPU halt when no other backend is selected |
//! | `panic-uart0` | Write message to UART0 DATA, then halt |
//!
//! # Usage
//!
//! ```ignore
//! use hisi_panic_handler as _;
//! ```

#![no_std]

#[cfg(all(feature = "panic-halt", not(feature = "panic-uart0")))]
mod halt;
#[cfg(feature = "panic-uart0")]
mod uart0;
