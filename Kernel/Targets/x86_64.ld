ENTRY(EntryAddress)
OUTPUT_FORMAT(elf64-x86-64)

PhysicalAddress = 0x80000000;
VirtualAddress = 0xfffffff800000000;

EntryAddress = Main - VirtualAddress;

SECTIONS 
{
    . = PhysicalAddress + VirtualAddress;

    .text BLOCK(4K) : AT(ADDR(.text) - VirtualAddress)
    {
        *(.text)
    }

    .rodata BLOCK(4K) : AT(ADDR(.rodata) - VirtualAddress)
    {
        *(.rodata)
    }

    .data BLOCK(4K) : AT(ADDR(.data) - VirtualAddress)
    {
        *(.data)
    }

    .bss BLOCK(4K) : AT(ADDR(.bss) - VirtualAddress)
    {
        *(.bss)
    }
}