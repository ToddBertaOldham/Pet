# Tools

## Standard Cross Tools

GCC and Binutils will need to be built for cross compilation. The tools listed below are necessary for building them.

* GCC
* G++
* GNU Make
* Flex
* GNU GMP
* GNU MPFR
* ISL
* CLooG 
* GNU MPC
* Texinfo
* GNU Bison

Using this command should install all necessary packages on Ubuntu based systems. The package "build-essential" should also cover many of these.
``` 
sudo apt install gcc g++ make flex libgmp3-dev libmpfr-dev libisl-dev libcloog-isl-dev libmpc-dev texinfo bison 
```
 Using this command with TARGET as a supported architecture (currently only x86_64) will download and build the source for you. It will take a very long time.

```
mkdir Toolchain
cd Toolchain
./../Scripts/SetupCrossTools.sh TARGET
```
Cross tools should be built in the Toolchain folder so the build system can find them. More information on the building the cross tools can be found [here](https://wiki.osdev.org/GCC_Cross-Compiler).

## UEFI
Building for UEFI requires MinGW-w64. It can be installed on Ubuntu using the following command.
```
sudo apt install mingw-w64
```
The cross tools script does not support building the MinGW-w64 compiler at the moment.