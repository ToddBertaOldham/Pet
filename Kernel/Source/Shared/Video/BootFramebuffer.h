// *************************************************************************
// BootFramebuffer.h
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#ifndef PET_KERNEL_SHARED_VIDEO_BOOTFRAMEBUFFER_HEADER
#define PET_KERNEL_SHARED_VIDEO_BOOTFRAMEBUFFER_HEADER

#include <stdint.h>

void SetBootFramebuffer(int width, int height, uint32_t *address);

int GetBootFramebufferWidth();
int GetBootFramebufferHeight();
uint32_t GetBootFramebufferPixel(int x, int y);
void SetBootFramebufferPixel(int x, int y, uint32_t color);
void ClearBootFramebuffer(uint32_t color);

void RegisterBootFramebuffer();
void UnregisterBootFramebuffer();

#endif