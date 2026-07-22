# hisi-panic-handler

Panic handlers for HiSilicon WS63 bare-metal Rust firmware.

Independent crate — follows the `panic-halt` / `panic-semihosting` / `panic-probe`
pattern. The panic output channel is an application decision, not a HAL concern.

## Features

| Feature | Behaviour |
|---------|-----------|
| `panic-halt` (default) | Silent CPU halt when no other backend is selected |
| `panic-uart0` | Write message + file:line to UART0, then halt |

`panic-uart0` uses the WS63 UART0 address and the flashboot-configured TCXO
console. Other chips must use `panic-halt` until they provide a verified adapter.

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
