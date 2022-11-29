
Current Memory MAP
---

```
            ┌─────────────────────────────────┐
            │                                 │
0x0000_0000 │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │        FLASH MEMORY        │  │
            │ │                            │  │
0x0003_2000 │ └────────────────────────────┘  │
            │                                 │
            │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │          RESERVED          │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ └────────────────────────────┘  │
            │                                 │
0x0200_0000 │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │            CLINT           │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
            │ │                            │  │
0x0200_FFFF │ └────────────────────────────┘  │
            │                                 │
            └─────────────────────────────────┘
```

Limitations
---

The current implementation doesn't support vectored trap hanlers.

Environment Calls
---

You can halt the execution at any time writing `255` to the `x15` register and executing `ecall`

```asm
li x15, 255 // send halt signal
ecall
```

Rust on the RV32i
---

Install the toolchain:

```sh
rustup target add rv32i-unknown-none-elf
```
