
Current Memory MAP
---

```
            ┌─────────────────────────────────┐
0x0000_0000 │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │        FLASH MEMORY        │  │
            │ │                            │  │
0x0003_2000 │ └────────────────────────────┘  │
            │ ┌────────────────────────────┐  │
            │ │          RESERVED          │  │
            │ └────────────────────────────┘  │
0x0200_0000 │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │                            │  │
            │ │            CLINT           │  │
            │ │                            │  │
            │ │                            │  │
0x0200_FFFF │ └────────────────────────────┘  │
            │ ┌────────────────────────────┐  │
            │ │          RESERVED          │  │
            │ └────────────────────────────┘  │
0x0C00_0000 │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │                            │  │
            │ │            PLIC            │  │
            │ │                            │  │
            │ │                            │  │
0x1000_0000 │ └────────────────────────────┘  │
            │ ┌────────────────────────────┐  │
            │ │          RESERVED          │  │
            │ └────────────────────────────┘  │
0x1001_3000 │ ┌────────────────────────────┐  │
            │ │                            │  │
            │ │                            │  │
            │ │           UART0            │  │
            │ │                            │  │
            │ │                            │  │
0x1001_3FFF │ └────────────────────────────┘  │
            └─────────────────────────────────┘
```

Limitations
---

The current implementation doesn't support vectored trap hanlers.

Environment Calls
---

HALT
------

You can halt the execution at any time writing `255` to the `x10` register and executing `ecall`

```asm
li x10, 255 // send halt signal
ecall
```

MEMDUMP
-----

You can dump a section of memory to a a writer at any time writing the `from` to `x11` the `to` to `x12` and `254` to `x10`. The default emulator will write dumps to a file and provides the argument `--dump-folder` to define the location of the dumps, if not provided it will use the current directory.

```asm
// Dumps the flash memory
li x11, 0
li x12, 0x0003_2000
li x10, 254
ecall
```

Rust on the RV32i
---

Install the toolchain:

```sh
rustup target add rv32i-unknown-none-elf
```
