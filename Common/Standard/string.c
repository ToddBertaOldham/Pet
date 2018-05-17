// *************************************************************************
// string.c
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************'

#include <string.h>
#include <stdint.h>

void *memcpy(void *destination, const void *source, size_t count) 
{
    uint8_t *destinationBytes = (uint8_t*)destination;
    const uint8_t *sourceBytes = (const uint8_t*)source;

    for (size_t i = 0; i < count; i++)
    {
        destinationBytes[i] = sourceBytes[i];
    }

    return destination;
}

void *memset(void *destination, int value, size_t count) 
{
    uint8_t *destinationBytes = (uint8_t*)destination;
    uint8_t trueValue = (uint8_t)value;

    for (size_t i = 0; i < count; i++)
    {
        destinationBytes[i] = trueValue;
    }

    return destination;
}