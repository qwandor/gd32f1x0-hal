//! Serial interface circular DMA RX transfer test

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::{asm, singleton};

use cortex_m_rt::entry;
use gd32f1x0_hal::{
    dma::Half,
    gpio::{OutputMode, PullMode},
    pac,
    prelude::*,
    serial::{Config, Serial},
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut flash = p.fmc.constrain();
    let mut rcu = p.rcu.constrain();

    let clocks = rcu.cfgr.freeze(&mut flash.ws);

    let channels = p.dma.split(&mut rcu.ahb);

    let mut gpioa = p.gpioa.split(&mut rcu.ahb);
    // let mut gpiob = p.gpiob.split(&mut rcu.ahb);

    // USART0
    let tx = gpioa
        .pa9
        .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);
    let rx = gpioa
        .pa10
        .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);

    // USART1
    // let tx = gpioa
    //     .pa2
    //     .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);
    // let rx = gpioa
    //     .pa3
    //     .into_alternate(&mut gpioa.config, PullMode::Floating, OutputMode::PushPull);

    let serial = Serial::usart(
        p.usart0,
        (tx, rx),
        Config::default().baudrate(9_600.bps()),
        clocks,
        &mut rcu.apb2,
    );

    let rx = serial.split().1.with_dma(channels.2);
    let buf = singleton!(: [[u8; 8]; 2] = [[0; 8]; 2]).unwrap();

    let mut circ_buffer = rx.circ_read(buf);

    while circ_buffer.readable_half().unwrap() != Half::First {}

    let _first_half = circ_buffer.peek(|half, _| *half).unwrap();

    while circ_buffer.readable_half().unwrap() != Half::Second {}

    let _second_half = circ_buffer.peek(|half, _| *half).unwrap();

    asm::bkpt();

    #[allow(clippy::empty_loop)]
    loop {}
}
