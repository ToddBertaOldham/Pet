// *************************************************************************
// BootData.h
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#ifndef PET_BOOT_BOOTDATA_HEADER
#define PET_BOOT_BOOTDATA_HEADER

#include <stdint.h>
#include <stddef.h>

struct BootFramebuffer
{
    uint64_t Address;
    uint64_t Size;
    uint64_t Width;
    uint64_t Height;
    uint64_t PixelsPerScanLine;
};

struct BootData 
{
    struct BootFramebuffer Framebuffer;
};

#endif