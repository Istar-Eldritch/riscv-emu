# sends the string "hello world\n" to uart0

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
  call setup_uart

  li a1, UART0_ADDR
  la a2, msg
  call print

  li x15, 255 # send halt signal
  ecall


print:
  lb t0, 0(a2)
  beqz t0, end_print
  sw t0, UART_TXDATA(a1)
  addi a2, a2, 1
  tail print
end_print:
  ret


setup_uart:
  # Enable UART0 TX
  li t0, UART0_ADDR
  li t1, 0x00001
  sw t1, UART_TXCTRL(t0)
  # No need to set the divider with the fake terminal, currently the bound rate is not emulated properly
  ret

msg:
  .string "hello world\n"

