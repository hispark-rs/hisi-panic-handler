//! Panic handlers for HiSilicon RISC-V chips.
//!
//! # Features
//!
//! | Feature | Behaviour |
//! |---------|-----------|
//! | `panic-halt` (default) | Silent CPU halt |
//! | `panic-uart0` | Write message to UART0 DATA, then halt |
//!
//! # Usage
//!
//! ```ignore
//! use hisi_panic_handler as _;
//! ```

#![no_std]

#[cfg(feature = "panic-halt")]
mod halt;
#[cfg(feature = "panic-uart0")]
mod uart0;
