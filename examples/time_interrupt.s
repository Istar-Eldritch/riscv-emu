# Setup a time interrupt in 10 cycles

.globl _start

_start:
  # setup interrupt handler
  la t0, handle_trap 
  csrw mtvec, t0

  li t0, 0x8
  csrs mstatus, t0 # SET MIE = 1
  li t0, 0x200bff8 # mem map of time on clint
  li t1, 0x2004000 # mem map of timecmp on clint

  lw t2, 0(t0) # load time
  addi t2, t2, 0xa # add 10 to the time
  sw t2, 0(t1) # save updated time to timecmp

  li t0, 0x80 
  csrs mie, t0 # Set MTIE = 1

  wfi

  li x15, 255 # send halt signal
  ecall

handle_trap:
  mret

