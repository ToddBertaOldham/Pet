# Pet

## About
Pet is a hobby OS and my pet project. It is still very early in development. The current focus is on x86_64 and UEFI.

## Building
Make sure [xbuild 0.5.4](https://github.com/rust-osdev/cargo-xbuild) is installed and then run the command below from the root project directory to build the kernel and UEFI boot loader.
```
./Scripts/BuildAll.sh
```
After everything has finished building, run the following from the root project directory to create an image.
```
./Scripts/MakeImage.sh
```

## License
Licensed under [MIT](LICENSE).