MEMORY
{
    /*
        MEMORY_MAP = [[0x00000000, 0x00010000, "PADDING"],
                  [0x3C000000, 0x3C800000, "DROM"],
                  [0x3FC80000, 0x3FCE0000, "DRAM"],
                  [0x3FC88000, 0x3FD00000, "BYTE_ACCESSIBLE"],
                  [0x3FF00000, 0x3FF20000, "DROM_MASK"],
                  [0x40000000, 0x40060000, "IROM_MASK"],
                  [0x42000000, 0x42800000, "IROM"],
                  [0x4037C000, 0x403E0000, "IRAM"],
                  [0x50000000, 0x50002000, "RTC_IRAM"],
                  [0x50000000, 0x50002000, "RTC_DRAM"],
                  [0x600FE000, 0x60100000, "MEM_INTERNAL2"]]
    */
    /*
        https://github.com/espressif/esptool/blob/master/esptool.py#L1919
        IROM_MAP_START = 0x42000000
        IROM_MAP_END   = 0x42800000
        DROM_MAP_START = 0x3c000000
        DROM_MAP_END   = 0x3c800000
    */
    IRAM : ORIGIN = 0x4037C000, LENGTH = 128k
    IROM : ORIGIN = 0x42000020, LENGTH = 0x800000
}


REGION_ALIAS("REGION_TEXT", IRAM);
REGION_ALIAS("REGION_RODATA", IRAM);
REGION_ALIAS("REGION_DATA", IRAM);
REGION_ALIAS("REGION_BSS", IRAM);
REGION_ALIAS("REGION_HEAP", IRAM);
REGION_ALIAS("REGION_STACK", IRAM);