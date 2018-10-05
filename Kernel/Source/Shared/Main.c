// *************************************************************************
// Main.c
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#include "BootData.h"
#include "Video/BootFramebuffer.h"

void Main(struct BootData *bootData)
{
    SetBootFramebuffer(bootData->Framebuffer.Width, bootData->Framebuffer.Height, (uint32_t*)bootData->Framebuffer.Address);
    
    ClearBootFramebuffer(0xFFFFFFFF);
}