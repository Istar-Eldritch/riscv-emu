# echoes the uppercase input to uart0 

#define UART0_ADDR 0x10013000

#define UART_TXDATA 0x00
#define UART_RXDATA 0x04
#define UART_TXCTRL 0x08
#define UART_RXCTRL 0x0c
#define UART_IE     0x10
#define UART_IP     0x14
#define UART_DIV    0x18


.globl _start

_start:
  # setup interrupt handler
  la t0, trap 
  csrw mtvec, t0
  call setup_uart
wait_for_input:
  wfi
  tail wait_for_input

setup_uart:
  # Enable UART0 TX
  li t0, UART0_ADDR
  li t1, 0x00001
  sw t1, UART_TXCTRL(t0)
  li t1, 0x00001
  sw t1, UART_RXCTRL(t0)
  li t1, 0x10
  sw t1, UART_IE(t0)
  # No need to set the divider with the fake terminal, currently the bound rate is not emulated properly

  li t0, 0x8
  csrs mstatus, t0 # SET MIE = 1
  li t0, 0x800
  csrs mie, t0 # Enable MEIE

  ret

echo_upper:
  lw a0, UART_RXDATA(a1)
  andi a0, a0, 0xff
  li t0, 97 # The ascii 'a'
  blt a0, t0, echo # bail if lt 'a'
  li t0, 122 # The ascii 'z'
  bgt a0, t0, echo # bail if gt 'z'
  li t0, 32
  sub a0, a0, t0
echo:
  sw a0, UART_TXDATA(a1)
  ret

trap:
  li a1, UART0_ADDR
  lw t1, UART_IP(a1)
  li t0, 0b10
  bne t0, t1, end_trap
  call echo_upper
end_trap:
  mret

