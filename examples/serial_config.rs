//! Serial interface loopback test
//!
//! You have to short the TX and RX pins to make this program work

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use gd32f1x0_hal::{
    gpio::{OutputMode, PullMode},
    pac,
    prelude::*,
    serial::{Config, Serial, StopBits},
};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate.
    let p = pac::Peripherals::take().unwrap();

    // Take ownership of the RCU peripheral and convert it into the corresponding HAL struct.
    let mut rcu = p.RCU.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`.
    let clocks = rcu.cfgr.freeze(&p.FMC.ws);

    // Prepare the GPIOA peripheral
    let mut gpioa = p.GPIOA.split(&mut rcu.ahb);

    // USART0
    // let tx = gpioa
    //     .pa9
    //     .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);
    // let rx = gpioa
    //     .pa10
    //     .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);

    // USART1
    // Configure pa2 and pa3 in alternate function mode for the USART.
    let tx = gpioa
        .pa2
        .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);
    let rx = gpioa
        .pa3
        .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);

    // Set up the usart device. Takes ownership of the USART registers and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let serial = Serial::usart(
        p.USART1,
        (tx, rx),
        Config::default()
            .baudrate(9600.bps())
            .stopbits(StopBits::STOP2)
            .parity_odd(),
        clocks,
        &mut rcu.apb1,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    let sent = b'U';
    block!(tx.write(sent)).ok();
    block!(tx.write(sent)).ok();

    loop {}
}
