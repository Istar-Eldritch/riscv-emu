.globl _start

_start:
  # setup interrupt handler
  la t0, handle_trap 
  csrw mtvec, t0

  li t0, 0x8
  csrs mstatus, t0 # SET MIE = 1
  csrs mie, t0 # Enable MSIE
  li t1, 1
  li t0, 0x2000000
  sw t1, 0(t0) # Trigger software interrupt
  wfi

handle_trap:
  li x15, 255 // send halt signal
  ecall
 
