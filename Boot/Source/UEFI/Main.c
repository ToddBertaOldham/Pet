// *************************************************************************
// Main.c
// Copyright 2018 Todd Berta-Oldham
// This code is licensed under MIT.
// *************************************************************************

#include "Uefi.h"
#include "Protocol/SimpleFileSystem.h"
#include "Protocol/GraphicsOutput.h"
#include "Guid/FileInfo.h"

#include "ELF64.h"
#include "BootData.h"

typedef void (*KernelMainFunction)(struct BootData*);

static EFI_HANDLE ImageHandle;
static EFI_SYSTEM_TABLE *SystemTable;
static UINTN MapKey;
static struct BootData KernelBootData;
static KernelMainFunction KernelEntryPoint;
static UINTN EventIndex;

EFI_STATUS GetFramebuffer() 
{
    EFI_GUID guid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;

    EFI_STATUS status;

    UINTN handleCount;
    EFI_HANDLE *handleBuffer;

    status = SystemTable->BootServices->LocateHandleBuffer(ByProtocol, &guid, NULL, &handleCount, &handleBuffer);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to locate GOP handle buffer.\r\n");
        return status;
    }

    EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;

    status = SystemTable->BootServices->OpenProtocol(handleBuffer[0], &guid, (void**)&gop, ImageHandle, NULL, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to open GOP protocol.\r\n");
        return status;
    }

    EFI_GRAPHICS_OUTPUT_MODE_INFORMATION *modeInformatonOuput;
    EFI_GRAPHICS_OUTPUT_MODE_INFORMATION *bestModeInformation;
    UINT32 bestModeIndex;

    for (UINT32 i = 0; i < gop->Mode->MaxMode; i++)
    {
        status = gop->QueryMode(gop, i, &gop->Mode->SizeOfInfo, &modeInformatonOuput);

        if (EFI_ERROR(status)) continue;
        if (modeInformatonOuput->PixelFormat == PixelBltOnly) continue;

        if (bestModeInformation != NULL) 
        {
            if (modeInformatonOuput->HorizontalResolution < bestModeInformation->HorizontalResolution) continue;
            if (modeInformatonOuput->VerticalResolution < bestModeInformation->VerticalResolution) continue;
        }

        bestModeInformation = modeInformatonOuput;
        bestModeIndex = i;
    }
    
    status = gop->SetMode(gop, bestModeIndex);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to update GOP mode.\r\n");
        return status;
    }

    KernelBootData.Framebuffer.Address = gop->Mode->FrameBufferBase;
    KernelBootData.Framebuffer.Size = gop->Mode->FrameBufferSize;
    KernelBootData.Framebuffer.Width = gop->Mode->Info->HorizontalResolution;
    KernelBootData.Framebuffer.Height = gop->Mode->Info->VerticalResolution;
    KernelBootData.Framebuffer.PixelsPerScanLine = gop->Mode->Info->PixelsPerScanLine;

    SystemTable->BootServices->CloseProtocol(handleBuffer[0], &guid, ImageHandle, NULL);
    SystemTable->BootServices->FreePool(handleBuffer);

    return EFI_SUCCESS;
}

EFI_STATUS LoadMemoryMap() 
{
    UINTN memoryMapSize = 0;
    EFI_MEMORY_DESCRIPTOR *memoryMap = NULL;
    UINTN descriptorSize = 0;
    UINT32 descriptorVersion = 0;

    // Get buffer size first. A empty pool must be allocated in order to retrieve the size. 

    EFI_STATUS status = SystemTable->BootServices->AllocatePool(EfiLoaderData, memoryMapSize, (void**)&memoryMap);
    if (EFI_ERROR(status)) 
    {        
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to allocate memory for dummy memory map buffer.\r\n");
        return status;
    }

    status = SystemTable->BootServices->GetMemoryMap(&memoryMapSize, memoryMap, &MapKey, &descriptorSize, &descriptorVersion);
    if (status != EFI_BUFFER_TOO_SMALL) 
    {        
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to get memory map buffer size.\r\n");
        return status;
    }

    SystemTable->BootServices->FreePool(memoryMap);
    
    // Allocate and retrieve the memory map.

    status = SystemTable->BootServices->AllocatePool(EfiLoaderData, memoryMapSize, (void**)&memoryMap);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to allocate memory for memory map buffer.\r\n");
        return status;
    }

    status = SystemTable->BootServices->GetMemoryMap(&memoryMapSize, memoryMap, &MapKey, &descriptorSize, &descriptorVersion);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to get memory map.\r\n");
        SystemTable->BootServices->FreePool(memoryMap);
        return status;
    }

    // Convert to our format.

    SystemTable->BootServices->FreePool(memoryMap);

    return EFI_SUCCESS;
}

EFI_STATUS LoadKernel() 
{
    EFI_GUID guid = EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID;

    EFI_STATUS status;

    UINTN handleCount;
    EFI_HANDLE *handleBuffer;

    status = SystemTable->BootServices->LocateHandleBuffer(ByProtocol, &guid, NULL, &handleCount, &handleBuffer);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to locate file system handle buffer.\r\n");
        return status;
    }

    EFI_HANDLE handle;
    EFI_SIMPLE_FILE_SYSTEM_PROTOCOL *sfs;
    EFI_FILE_PROTOCOL *volumeFileProtocol;
    EFI_FILE_PROTOCOL *kernelFileProtocol;

    for (UINTN i = 0; i < handleCount; i++)
    {
        handle = handleBuffer[i];

        status = SystemTable->BootServices->OpenProtocol(handle, &guid, (void**)&sfs, ImageHandle, NULL, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
        if (EFI_ERROR(status)) 
        {
            SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to open file system protocol.\r\n");
            return status;
        }

        status = sfs->OpenVolume(sfs, &volumeFileProtocol);
        if (EFI_ERROR(status)) 
        {
            SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to open file system volume.\r\n");
            return status;
        }

        status = volumeFileProtocol->Open(volumeFileProtocol, &kernelFileProtocol, L"System\\Kernel.sys", EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);     
        
        if (status == EFI_SUCCESS) 
        {
            break;
        }
        else
        {
            SystemTable->BootServices->CloseProtocol(handle, &guid, ImageHandle, NULL);

            if (status != EFI_NOT_FOUND && EFI_ERROR(status)) 
            {
                SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to open kernel file.\r\n");
                return status;
            }
        }
    }

    if (kernelFileProtocol == NULL)
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to find kernel file.\r\n");
        return EFI_NOT_FOUND;
    }

    SystemTable->BootServices->FreePool(handleBuffer);

    // Get file info size.

    UINTN fileInfoSize = 0;
    EFI_FILE_INFO *fileInfo;
    EFI_GUID fileInfoGUID = EFI_FILE_INFO_ID;

    status = kernelFileProtocol->GetInfo(kernelFileProtocol, &fileInfoGUID, &fileInfoSize, NULL);
    if (status != EFI_BUFFER_TOO_SMALL) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to get kernel file info buffer size.\r\n");
        return status;
    }

    // Get file info.

    UINT64 bufferSize;

    status = SystemTable->BootServices->AllocatePool(EfiLoaderData, fileInfoSize, (void**)&fileInfo);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to allocate kernel file info buffer.\r\n");
        return status;
    }

    status = kernelFileProtocol->GetInfo(kernelFileProtocol, &fileInfoGUID, &fileInfoSize, fileInfo);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to get kernel file info.\r\n");
        return status;
    }

    bufferSize = fileInfo->FileSize;

    SystemTable->BootServices->FreePool(fileInfo);

    // Load kernel file.

    void *buffer;

    status = SystemTable->BootServices->AllocatePool(EfiLoaderData, bufferSize, &buffer);
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to allocate memory for kernel file.\r\n");
        return status;
    }

    status = kernelFileProtocol->Read(kernelFileProtocol, &bufferSize, buffer);  
    if (EFI_ERROR(status)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to read kernel file.\r\n");
        return status;
    }

    SystemTable->BootServices->CloseProtocol(handle, &guid, ImageHandle, NULL);

    // Load ELF64 image.

    struct ELF64Header *elf64Header;
    GetELF64Header(buffer, &elf64Header);

    if (!IsELF64(elf64Header)) 
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Kernel file is invalid.\r\n");
        return EFI_LOAD_ERROR;
    }

    LoadELF64Physical(buffer);

    KernelEntryPoint = (KernelMainFunction)elf64Header->Entry;

    SystemTable->BootServices->FreePool(buffer);

    return EFI_SUCCESS;
}

static void PrintHeader() 
{
    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Pet UEFI Bootloader\r\n");
    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Copyright 2018 Todd Berta-Oldham\r\n");
}

static void WaitForUserInput()
{
    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Press any key to continue...\r\n");
    SystemTable->BootServices->WaitForEvent(1, &SystemTable->ConIn->WaitForKey, &EventIndex);
}

EFI_STATUS Main(EFI_HANDLE imageHandle, EFI_SYSTEM_TABLE *systemTable) 
{
    ImageHandle = imageHandle;
    SystemTable = systemTable;

    // Apply framebuffer with best video mode.

    EFI_STATUS status = GetFramebuffer();
    if (EFI_ERROR(status)) 
    {
        PrintHeader();
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Failed to get framebuffer. Cannot boot Pet.\r\n");
        WaitForUserInput();
        return status;
    }
    
    PrintHeader();

    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Successfully obtained framebuffer.\r\n");


    // Load kernel from disk.

    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Loading kernel...\r\n");
    
    status = LoadKernel();
    if (EFI_ERROR(status))
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Loading kernel failed.\r\n");
        WaitForUserInput();
        return status;
    }

    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Successfully loaded kernel.\r\n");


    // Retrieve and convert memory map.

    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Loading memory map...\r\n");

    status = LoadMemoryMap();
    if (EFI_ERROR(status))
    {
        SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Loading memory map failed.\r\n");
        WaitForUserInput();
        return status;
    }

    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Successfully loaded memory map.\r\n");


    // Exit boot services and jump to kernel.
    
    SystemTable->ConOut->OutputString(SystemTable->ConOut, L"Jumping to kernel...\r\n");

    SystemTable->BootServices->ExitBootServices(ImageHandle, MapKey);

    KernelEntryPoint(&KernelBootData);

    while (true) { }

    return EFI_SUCCESS;
}