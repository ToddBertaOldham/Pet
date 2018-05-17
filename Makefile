# *************************************************************************
# Makefile
# Copyright 2018 Todd Berta-Oldham
# This code is licensed under MIT.
# *************************************************************************

# Base Variables.

ROOTPATH = $(CURDIR)/Root
IMAGESPATH = Images

PROJECTS = Boot Kernel

# Set defaults.

ifndef PLATFORM
	PLATFORM = PC
endif

ifndef BOOTPLATFORM
	BOOTPLATFORM = UEFI
endif

ifndef ARCH 
	ARCH = x86_64
endif

# Set variables for sub-makefiles.

MAKEVARIABLES = BOOTPLATFORM=$(BOOTPLATFORM) ARCH=$(ARCH) ROOTINSTALL=$(ROOTPATH)

# Targets.

.PHONY: all root image clean

all: 
	$(foreach project, $(PROJECTS), $(MAKE) -C $(project) $(MAKEVARIABLES) &&) true

root: all
	mkdir -p Root
	$(foreach project, $(PROJECTS), $(MAKE) -C $(project) $(MAKEVARIABLES) root &&) true

image: root
	mkdir -p Images/
	rm -f Images/Boot.img Images/Pet.iso 

	dd if=/dev/zero of=Images/Boot.img bs=1M count=100
	mkfs.fat -F 32 Images/Boot.img 

	mmd -i Images/Boot.img ::/System
	mcopy -i Images/Boot.img Root/System/Kernel.sys ::/System
	mmd -i Images/Boot.img ::/EFI
	mmd -i Images/Boot.img ::/EFI/BOOT
	mcopy -i Images/Boot.img Root/EFI/BOOT/BOOTX64.EFI ::/EFI/BOOT
	
	xorriso -as mkisofs -R -f -e Boot.img -no-emul-boot -o Images/Pet.iso Images/

clean:
	rm -rf "$(ROOTPATH)" $(IMAGESPATH)
	$(foreach project, $(PROJECTS), $(MAKE) -C $(project) $(MAKEVARIABLES) clean &&) true