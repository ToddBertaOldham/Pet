# Verdure OS

## About
Verdure OS is an operating system written in Rust. It is still very early in development. The current focus is on x86_64 and UEFI.

## Directory
* boot_apps - Boot time applications (like the boot loader) that run on top of firmware/bare metal.
* kernel - The microkernel core of Verdure OS. Manages memory, tasks, and IPC.
* libraries - The building blocks of both the kernel and all applications.
* scripts - Some scripts for building the OS and creating a disc image.
* user_apps - All userspace applications including services and drivers. Coming soon™

## Building
Make sure [Rust](https://www.rust-lang.org/tools/install) is installed and then run the command below from the root project directory to build the kernel and boot loader.
```
./scripts/build_all.sh
```
A script is also available for building on Windows.
```
./scripts/build_all.ps1
```
After everything has finished building, run the following from the root project directory to create a disc image. This step requires mtools and xorriso to be installed. [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) is required for this step on Windows.
```
./scripts/make_image.sh
```

## License
Licensed under [MIT](LICENSE).