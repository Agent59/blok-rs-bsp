#![no_std]

pub use rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use hal::entry;

#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub use hal::pac;

hal::bsp_pins!(
    Gpio0 { 
        name: tx,
        aliases: { FunctionUart: Gp0Uart0Tx }
    },
    Gpio1 {
        name: rx,
        aliases: { FunctionUart: Gp0Uart0Rx }
    },
    Gpio16 {
        name: sda,
        aliases: { FunctionI2C: Gp16I2C0Sda}
    },
    Gpio17 {
        name: scl,
        aliases: { FunctionI2C: Gp17I2C0Scl }
    },
    
    Gpio4 {
        name: gpio4,
    },
    Gpio5 {
        name: gpio5,
    },
    Gpio6 {
        name: gpio6,
    }
    Gpio7 {
        name: gpio7,
    },
    
    Gpio8 {
        name: gpio8,
        aliases: { FunctionUart: Gp8Uart0Tx }
    },
    Gpio9 {
        name: gpio9,
        aliases: { FunctionUart: Gp9Uart0Rx}
    },
    
    Gpio29 {
        name: gpio29,
    },
    Gpio28 {
        name: gpio28,
    },
    Gpio27 {
        name: gpio27,
    },
    Gpio26 {
        name: gpio26,
    },
    
    Gpio22 {
        name: gpio22,
        aliases: { FunctionSpi: Gp22Spi0Sck }
    },
    Gpio20 {
        name: gpio20,
        aliases: { FunctionSpi: Gp20Spi0Rx }
    },
    Gpio23 {
        name: gpio23,
        aliases: { FunctionSpi: Gp23Spi0Tx }
    },
    Gpio21 {
        name: gpio21,
        aliases: { FunctionSpi: Gp21Spi0Csn }
    },
    
    Gpio25 {
        name: led,
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
