// *************************************************************************
// Main.c
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#include "BootData.h"

void Main(struct BootData *bootData)
{
    uint32_t *framebuffer = (uint32_t*)bootData->Framebuffer.Address;
    size_t area = bootData->Framebuffer.Width * bootData->Framebuffer.Height;

    for (size_t i = 0; i < area; i++)
    {
        framebuffer[i] = 0xFFFFFFFF;
    }
}