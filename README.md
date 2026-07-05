# hisi-panic-handler

Panic handlers for HiSilicon RISC-V chips (WS63 / BS2X).

Independent crate — follows the `panic-halt` / `panic-semihosting` / `panic-probe`
pattern. The panic output channel is an application decision, not a HAL concern.

## Features

| Feature | Behaviour |
|---------|-----------|
| `panic-halt` (default) | Silent CPU halt |
| `panic-uart0` | Write message + file:line to UART0, then halt |

## Usage

```toml
# Silent halt
hisi-panic-handler = "0.1"

# Visible panic messages on UART0
hisi-panic-handler = { version = "0.1", default-features = false, features = ["panic-uart0"] }
```

```rust
use hisi_panic_handler as _;
```
