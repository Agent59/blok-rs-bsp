# blok-rs-bsp
A Board Support Package for the RP2040 based Blok microcontroller.

## The contents of this project have been moved to the [`rp-hal-boards`](https://github.com/rp-rs/rp-hal-boards) repository <br> under the name [`boardsource-blok`](https://github.com/rp-rs/rp-hal-boards/tree/main/boards/boardsource-blok).
<br>

## Run the rainbow example

To run the example, ensure you have the latest stable
version of Rust installed, along with the right target support:

```sh
rustup self update
rustup update stable
rustup target add thumbv6m-none-eabi
```
Clone this repository and run
```sh
cargo build --release --example rainbow
```

You will get an ELF file called
`./target/thumbv6m-none-eabi/release/examples/rainbow`

If you want to convert the ELF file to a UF2 and automatically copy it to the
USB drive exported by the RP2040 bootloader
you need to install [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs):

```sh
cargo install elf2uf2-rs
```
and then simply boot your board into
bootloader mode and run:

```sh
cargo run --release --example rainbow
```
