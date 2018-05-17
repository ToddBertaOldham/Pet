#!/bin/bash
#SetupCrossTools.sh

# Based off of https://wiki.osdev.org/GCC_Cross-Compiler

# Start

set -e 

GCCVersion=7.2.0 
BINUTILSVERSION=2.29

WorkingDirectory=$(pwd)
DownloadsDirectory="$WorkingDirectory/Downloads"
BuildDirectory="$WorkingDirectory/Build"

# Functions

function print_usage() 
{
    echo "Usage: ./SetupCrossTools.sh TARGET"
    echo "TARGET can only be x86_64 at the moment."
}

function check_tool() 
{
    if ! [ -x "$(command -v $1)" ]; then
        echo "Error: $1 is not installed." >&2
        exit 1
    fi
}

function download() 
{
    echo "Downloading $1..."
    wget -nc $1 -P "$DownloadsDirectory"
}

function unpack()
{
    echo "Unpacking $1..."
    tar -x -f "$DownloadsDirectory/$1" -C "$DownloadsDirectory" --skip-old-files
}

function buildBinutils()
{
    echo "Building BinUtils for $1."

    outputPath="$WorkingDirectory/$1"
    mkdir -p $outputPath

    binutilsPath="$BuildDirectory/binutils_$1"
    mkdir -p $binutilsPath

    export PREFIX=$outputPath
    export TARGET=$1
    export PATH="$PREFIX/bin:$PATH"

    cd $binutilsPath

    $DownloadsDirectory/binutils-$BINUTILSVERSION/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror

    make
    make install
}

function buildGCC() 
{
    echo "Building GCC for $1."

    outputPath="$WorkingDirectory/$1"
    mkdir -p $outputPath

    gccPath="$BuildDirectory/gcc_$1"
    mkdir -p $gccPath

    export PREFIX=$outputPath
    export TARGET=$1
    export PATH="$PREFIX/bin:$PATH"

    cd $gccPath

    $DownloadsDirectory/gcc-$GCCVersion/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers

    make all-gcc
    make all-target-libgcc
    make install-gcc
    make install-target-libgcc
}

# Check Tools

check_tool make
check_tool tar
check_tool wget

# Handle Parameters

if [ "$1" = "x86_64" ]; then
    echo "Building tools for x86_64."
    GCCTARGETS=(x86_64-elf)
    BINUTILSTARGET=(x86_64-elf)
else 
    echo "Error: Target not specified." >&2
    print_usage
    exit 1
fi 

echo "This may take hours to complete."

# Create Directories

mkdir -p $DownloadsDirectory
mkdir -p $BuildDirectory

# Downloads

download ftp://ftp.gnu.org/gnu/gcc/gcc-$GCCVersion/gcc-$GCCVersion.tar.gz
download ftp://ftp.gnu.org/gnu/binutils/binutils-$BINUTILSVERSION.tar.gz

# Unpack

unpack gcc-$GCCVersion.tar.gz
unpack binutils-$BINUTILSVERSION.tar.gz

# Build

for binutilsTarget in ${BINUTILSTARGET[@]}
do
    buildBinutils $binutilsTarget
done

for gccTarget in ${GCCTARGETS[@]}
do
    buildGCC $gccTarget
done