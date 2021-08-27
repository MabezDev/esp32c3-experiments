INCLUDE esp32c3-memory.x

SECTIONS
{
  .header : 
  {
    LONG(0xaedb041d)
    LONG(0xaedb041d)
  } > HEADER

}

INCLUDE link.x

uart_tx_one_char = 0x40000068;
ets_delay_us = 0x40000050;