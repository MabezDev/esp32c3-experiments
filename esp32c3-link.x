INCLUDE esp32c3-memory.x

SECTIONS
{
  .header : AT(0)
  {
    LONG(0xaedb041d)
    LONG(0xaedb041d)
  } > IROM
}

_stext = ORIGIN(IROM) + 8;

INCLUDE riscv-link.x

uart_tx_one_char = 0x40000068;
ets_delay_us = 0x40000050;