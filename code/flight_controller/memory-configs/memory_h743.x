MEMORY
{
    FLASH             : ORIGIN = 0x08000000, LENGTH = 1920K /* CODE STORE*/
    SETTIINGS_STORAGE : ORIGIN = 0x081e0000, LENGTH = 128K /* SETTINGS STORE*/
    DTCM              : ORIGIN = 0x20000000, LENGTH = 128K /* Fast DTCM RAM for stack */
    RAM               : ORIGIN = 0x24000000, LENGTH = 512K /* AXI SRAM for general stuff */
    AHB_SRAM          : ORIGIN = 0x30000000, LENGTH = 288K /* AHB SRAM for DMA buffers */
}

/* Put stack in the fast DTCM RAM */
_stack_end   = ORIGIN(DTCM);
_stack_start = ORIGIN(DTCM) + LENGTH(DTCM);

SECTIONS
{
     .buffers (NOLOAD) : ALIGN(4) {
       *(.buffers);
       . = ALIGN(4);
     } > AHB_SRAM
}