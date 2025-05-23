MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH =  512K /* BANK_1_REGION_1 + BANK_1_REGION_2 + BANK_1_REGION_3 */
    RAM   : ORIGIN = 0x20000000, LENGTH =  128K /* SRAM */
}

SECTIONS
{
     .buffers (NOLOAD) : ALIGN(4) {
       *(.buffers);
       . = ALIGN(4);
     } > RAM
}