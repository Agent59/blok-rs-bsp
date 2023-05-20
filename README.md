# blok-rust-bsp
A Board Support Package for the RP2040 based Blok microcontroller.

This board support package is based
on the [`rp-hal`](https://github.com/rp-rs/rp-hal)
and the [`rp-hal-boards`](https://github.com/rp-rs/rp-hal-boards).
<br>
In the future this project may be added to the [`rp-hal-boards`](https://github.com/rp-rs/rp-hal-boards) repository.
<br><br><br>


## DISCLAIMER
This bsp is in a very early develompent phase and will probably be very unstable!
<br><br><br>


## Run the blinky example

To run the example, ensure you have the latest stable
version of Rust installed, along with the right target support:

```sh
rustup self update
rustup update stable
rustup target add thumbv6m-none-eabi
```
Clone this repository and run
```sh
cargo build --release --example blinky
```

You will get an ELF file called
`./target/thumbv6m-none-eabi/release/examples/blinky`

If you want to convert the ELF file to a UF2 and automatically copy it to the
USB drive exported by the RP2040 bootloader
you need to install [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs):

```sh
cargo install elf2uf2-rs
```
and then simply boot your board into
bootloader mode and run:

```sh
cargo run --release --example blinky
```
