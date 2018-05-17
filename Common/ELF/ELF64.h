// *************************************************************************
// ELF64.h
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************'

#ifndef PET_COMMON_ELF64_HEADER
#define PET_COMMON_ELF64_HEADER

#include <stddef.h>
#include <stdbool.h>
#include <string.h>

#include "ELF.h"

struct ELF64Header
{
    struct ELFIdentificationHeader Identification;
    uint16_t Type;
    uint16_t Architecture;
    uint32_t Version;
    uint64_t Entry;
    uint64_t ProgramHeaderOffset;
    uint64_t SectionHeaderOffset;
    uint32_t Flags;
    uint16_t ELFHeaderSize;
    uint16_t ProgramHeaderSize;
    uint16_t ProgramHeaderCount;
    uint16_t SectionHeaderSize;
    uint16_t SectionHeaderCount;
    uint16_t SectionHeaderStringTableIndex;
};

struct ELF64ProgramHeader
{
    uint32_t Type;
    uint32_t Flags;
    uint64_t Offset;
    uint64_t VirtualAddress;
    uint64_t PhysicalAddress;
    uint64_t FileSize;
    uint64_t MemorySize;
    uint64_t Alignment;    
};

struct ELF64SectionHeader
{
    uint32_t Name;
    uint32_t Type;
    uint64_t Flags;
    uint64_t Address;
    uint64_t Offset;
    uint64_t Size;
    uint32_t Link;
    uint32_t Info;
    uint64_t AddressAlign;
    uint64_t EntrySize;
};

void GetELF64Header(const void *buffer, struct ELF64Header **elf64Header);
void GetELF64ProgramHeader(const void *buffer, uint16_t index, struct ELF64ProgramHeader **elf64ProgramHeader);
void GetELF64SectionHeader(const void *buffer, uint16_t index, struct ELF64SectionHeader **elf64SectionHeader);

bool IsELF64(const struct ELF64Header *elf64Header);

void LoadELF64Physical(const void *buffer);
void LoadELF64Virtual(const void *buffer);

#endif