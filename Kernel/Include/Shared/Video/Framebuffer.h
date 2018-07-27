// *************************************************************************
// Framebuffer.h
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#ifndef PET_KERNEL_SHARED_VIDEO_FRAMEBUFFER_HEADER
#define PET_KERNEL_SHARED_VIDEO_FRAMEBUFFER_HEADER

#include <stdint.h>

struct Framebuffer 
{
    int (*GetWidth)();
    int (*GetHeight)();

    uint32_t (*GetPixel)(int x, int y);
    void (*SetPixel)(int x, int y, uint32_t color);
    void (*Clear)(uint32_t color);
};

void RegisterFramebuffer(const struct Framebuffer *framebuffer);
void UnregisterFramebuffer(const struct Framebuffer *framebuffer);

#endif