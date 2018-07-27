// *************************************************************************
// BootFramebuffer.c
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#include "BootFramebuffer.h"
#include "Video/Framebuffer.h"
#include <stddef.h>

static int Width;
static int Height;
static uint32_t *Address;

void SetBootFramebuffer(int width, int height, uint32_t *address)
{
    Width = width;
    Height = height;
    Address = address;
}

int GetBootFramebufferWidth() 
{
    return Width;
}
int GetBootFramebufferHeight() 
{
    return Height;
}
uint32_t GetBootFramebufferPixel(int x, int y) 
{
    return Address[y * Width + x];
}
void SetBootFramebufferPixel(int x, int y, uint32_t color) 
{
    Address[y * Width + x] = color;
}
void ClearBootFramebuffer(uint32_t color) 
{
    size_t length = Width * Height;  
    for(size_t i = 0; i < length; i++)
    {
        Address[i] = color;
    } 
}

void RegisterBootFramebuffer()
{

}
void UnregisterBootFramebuffer() 
{

}