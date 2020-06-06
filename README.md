# Verdure

## About
Verdure is an operating system written in Rust. It is still very early in development. The current focus is on x86_64 and UEFI.

## Building
Make sure [Rust](https://www.rust-lang.org/tools/install) and [Cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) are installed and then run the command below from the root project directory to build the kernel and UEFI boot loader.
```
./scripts/build_all.sh
```
A batch script is also available for building on Windows.
```
./scripts/build_all.bat
```
After everything has finished building, run the following from the root project directory to create a disc image. This step requires [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) when on Windows and requires mtools and xorriso to be installed.
```
./scripts/make_image.sh
```

## License
Licensed under [MIT](LICENSE).