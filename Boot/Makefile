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

CFLAGS = -I "Include/" -I "Source/Shared/" -I "Source/Shared/Standard/" -I "../Common/ELF/" -I "../Common/Standard/" -Wall -Wextra -ffreestanding -fno-stack-protector -fpic -fshort-wchar -mno-red-zone -mno-mmx -mno-sse -mno-sse2
SOURCES = $(wildcard Source/Shared/*.c)  $(wildcard Source/Shared/Standard/*.c)
COMMONSOURCES = $(wildcard ../Common/ELF/*.c) $(wildcard ../Common/Standard/*.c)

# Set defaults.

ifndef ROOTINSTALL
	ROOTINSTALL = $(DEFAULTROOTINSTALL)
endif

ifndef BOOTPLATFORM
	BOOTPLATFORM = UEFI
endif

ifndef ARCH 
	ARCH = x86_64
endif

# Handle platform and architecture.

ifeq ($(BOOTPLATFORM), UEFI)

	ROOTPATH = EFI/BOOT/

	CFLAGS += -I "Source/UEFI/" -I "../ThirdParty/UEFIHeaders/"
	LDFLAGS += -nostdlib --dll -shared --subsystem 10 -e Main
	SOURCES += $(wildcard $(SOURCEPATH)/UEFI/*.c)

	ifeq ($(ARCH), x86_64)

		BINARY = $(BINARIESPATH)/BOOTX64.EFI

		CC = x86_64-w64-mingw32-gcc
		CFLAGS += -I "../ThirdParty/UEFIHeaders/X64" 		
		LD = x86_64-w64-mingw32-ld
		
	endif

endif

# Create objects list.

OBJECTS = $(patsubst $(SOURCEPATH)/%.c,$(OBJECTSPATH)/%.o,$(SOURCES)) 
COMMONOBJECTS = $(patsubst ../Common/%.c,$(OBJECTSPATH)/Common/%.o,$(COMMONSOURCES)) 

# Targets

.PHONY: all root clean

all: $(BINARY)

$(BINARY): $(OBJECTS) $(COMMONOBJECTS)
	mkdir -p $(BINARIESPATH)
	$(LD) $(LDFLAGS) $(OBJECTS) $(COMMONOBJECTS) -o $(BINARY)

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