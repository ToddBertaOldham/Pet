// *************************************************************************
// ELF64.c
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************'

#include "ELF64.h"

void GetELF64Header(const void *buffer, struct ELF64Header **elf64Header)
{
    *elf64Header = (struct ELF64Header*)buffer;
}
void GetELF64ProgramHeader(const void *buffer, uint16_t index, struct ELF64ProgramHeader **elf64ProgramHeader) 
{
    struct ELF64Header *elf64Header;

    GetELF64Header(buffer, &elf64Header);

    *elf64ProgramHeader = (struct ELF64ProgramHeader*)(buffer + elf64Header->ProgramHeaderOffset + (elf64Header->ProgramHeaderSize * index));
}
void GetELF64SectionHeader(const void *buffer, uint16_t index, struct ELF64SectionHeader **elf64SectionHeader)
{
    struct ELF64Header *elf64Header;
    
    GetELF64Header(buffer, &elf64Header);

    *elf64SectionHeader = (struct ELF64SectionHeader*)(buffer + elf64Header->SectionHeaderOffset + (elf64Header->SectionHeaderSize * index));
}

bool IsELF64(const struct ELF64Header *elf64Header) 
{
    if (elf64Header->Identification.Magic0 != ELFMAGIC0 || elf64Header->Identification.Magic1 != ELFMAGIC1 || 
    elf64Header->Identification.Magic2 != ELFMAGIC2 || elf64Header->Identification.Magic3 != ELFMAGIC3)
    {
        return false;
    }

    if (elf64Header->Identification.Class != ELFCLASS_64BIT) 
    {
        return false;
    }

    return true;
}

static inline void CopyProgramSegment(const void *buffer, const struct ELF64ProgramHeader *programHeader, void *destination) 
{
    const void *source = buffer + programHeader->Offset;
    memcpy(destination, source, programHeader->FileSize);
    memset(destination + programHeader->FileSize, 0, programHeader->MemorySize - programHeader->FileSize);
}
void LoadELF64Physical(const void *buffer)
{
    struct ELF64Header *elf64Header;
    GetELF64Header(buffer, &elf64Header);

    for (uint16_t i = 0; i < elf64Header->ProgramHeaderCount; i++)
    {
        struct ELF64ProgramHeader *programHeader;
        GetELF64ProgramHeader(buffer, i, &programHeader);

        if (programHeader->Type != ELFPROGRAMHEADERTYPE_LOAD) continue;

        CopyProgramSegment(buffer, programHeader, (void*)programHeader->PhysicalAddress);
    }
}
void LoadELF64Virtual(const void *buffer)
{    
    struct ELF64Header *elf64Header;
    GetELF64Header(buffer, &elf64Header);
    
    for (uint16_t i = 0; i < elf64Header->ProgramHeaderCount; i++)
    {
        struct ELF64ProgramHeader *programHeader;
        GetELF64ProgramHeader(buffer, i, &programHeader);

        if (programHeader->Type != ELFPROGRAMHEADERTYPE_LOAD) continue;

        CopyProgramSegment(buffer, programHeader, (void*)programHeader->VirtualAddress);
    }
}