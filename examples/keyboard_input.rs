#![no_std]
#![no_main]

use core::iter::once;
use embedded_hal::timer::CountDown;
use panic_halt as _;
use blok_rs_bsp::{entry, hal};
use blok_rs_bsp::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pac::interrupt,
        pio::PIOExt,
        timer::Timer,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};
use usb_device::{
    bus::UsbBusAllocator,
    device::UsbDevice,
    device::UsbDeviceBuilder,
    device::UsbVidPid,
};
use usbd_hid::{
    hid_class::HIDClass,
    descriptor::KeyboardReport,
    descriptor::SerializedDescriptor,
};

// shared with the interrupt
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        USB_BUS = Some(usb_bus);
    }

    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    let mut usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 10);
    unsafe {
        USB_HID = Some(usb_hid);
    }

    let mut usb_device = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x1209, 0x0001))
        .product("keyboard input")
        .build();
    unsafe {
        USB_DEVICE = Some(usb_device);
    }


    // enable usb interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut i = 0;

    loop {
        if i == 0 {
            delay.delay_ms(5_000);
        }

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, //LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x0b, 0x00, 0x00, 0x00, 0x00, 0x00] // H
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, //LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x08, 0x00, 0x00, 0x00, 0x00, 0x00] // E
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, //LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00] // L
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00] // nothing
        });


        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, //LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00] // L
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0b0000_0010, //LeftShift
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x12, 0x00, 0x00, 0x00, 0x00, 0x00] // O
        });

        delay.delay_ms(100);
        push_report(KeyboardReport {
            modifier: 0x00,
            reserved: 0x00,
            leds: 0x00,
            keycodes: [0x2c, 0x00, 0x00, 0x00, 0x00, 0x00] // space
        });

        
        i += 1;
        if i >= 5 {
            hal::rom_data::reset_to_usb_boot(0, 0);
        }
    }
}

fn push_report(report: KeyboardReport) {
    let _ = critical_section::with(|_| unsafe {
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap();
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_device = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_device.poll(&mut [usb_hid]);
}
