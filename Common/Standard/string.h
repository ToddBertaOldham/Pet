// *************************************************************************
// string.h
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************'

#ifndef PET_C_STANDARD_LIBRARY_STRING_HEADER
#define PET_C_STANDARD_LIBRARY_STRING_HEADER

#include <stddef.h>

void *memcpy(void *destination, const void *source, size_t count);
void *memset(void *destination, int value, size_t count);

#endif