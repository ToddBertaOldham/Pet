# *************************************************************************
# Makefile
# Copyright 2018 Todd Berta-Oldham
# This code is licensed under MIT.
# *************************************************************************

# Base Variables.

SOURCEPATH = Source
BINARIESPATH = Binaries
OBJECTSPATH = Objects
DEFAULTROOTINSTALL = Root
ROOTPATH = System
BINARY = $(BINARIESPATH)/Kernel.sys

CFLAGS = -I "Include/Shared" -I "Source/Shared/" -I "Source/Shared/Standard/" -I "../Boot/Include/" -I "../Common/ELF/" -I "../Common/Standard/" -ffreestanding -mno-red-zone -Wall -Wextra -mno-mmx -mno-sse -mno-sse2
LDFLAGS += -nostdlib
SOURCES = $(wildcard Source/Shared/*.c) $(wildcard Source/Shared/**/*.c)
COMMONSOURCES = $(wildcard ../Common/ELF/*.c) $(wildcard ../Common/Standard/*.c)

# Set defaults.

ifndef ROOTINSTALL
	ROOTINSTALL = $(DEFAULTROOTINSTALL)
endif

ifndef PLATFORM
	PLATFORM = PC
endif

ifndef ARCH 
	ARCH = x86_64
endif

# Handle platform and architecture.

ifeq ($(ARCH), x86_64)

	TOOLSPREFIX = ../Toolchain/x86_64-elf/bin/x86_64-elf
	LDSCRIPT = Source/x86_64/Linker.ld
	SOURCES += $(wildcard Source/x86_64/*.c)

endif

CC = $(TOOLSPREFIX)-gcc
LD = $(TOOLSPREFIX)-ld

# Create objects list.

OBJECTS = $(patsubst $(SOURCEPATH)/%.c,$(OBJECTSPATH)/%.o,$(SOURCES)) 
COMMONOBJECTS = $(patsubst ../Common/%.c,$(OBJECTSPATH)/Common/%.o,$(COMMONSOURCES)) 

# Targets

.PHONY: all root clean

all: $(BINARY)

$(BINARY): $(OBJECTS) $(COMMONOBJECTS)
	mkdir -p $(BINARIESPATH)
	$(LD) $(LDFLAGS) -T $(LDSCRIPT) $(OBJECTS) $(COMMONOBJECTS) -o $(BINARY)

$(OBJECTSPATH)/%.o: $(SOURCEPATH)/%.c
	mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJECTSPATH)/Common/%.o: ../Common/%.c
	mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

root: all
	mkdir -p $(ROOTINSTALL)/$(ROOTPATH)
	cp $(BINARY) $(ROOTINSTALL)/$(ROOTPATH)

clean:
	rm -rf $(OBJECTSPATH) $(BINARIESPATH) $(DEFAULTROOTINSTALL)