//! Draw a square, circle and triangle on a 128x32px display.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Run on a Blue Pill with `cargo run --example graphics_i2c_128x32`, currently only works on
//! nightly.

#![no_std]
#![no_main]

#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate embedded_graphics;
extern crate embedded_hal as hal;
extern crate panic_semihosting;
extern crate ssd1306;
extern crate stm32f103xx_hal as blue_pill;

use blue_pill::i2c::{BlockingI2c, DutyCycle, Mode};
use blue_pill::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};
use rt::ExceptionFrame;
use ssd1306::prelude::*;
use ssd1306::Builder;

entry!(main);

fn main() -> ! {
    let dp = blue_pill::stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let yoffset = 8;

    disp.draw(
        Line::new(
            Coord::new(8, 16 + yoffset),
            Coord::new(8 + 16, 16 + yoffset),
        ).with_stroke(Some(1u8.into()))
        .into_iter(),
    );
    disp.draw(
        Line::new(Coord::new(8, 16 + yoffset), Coord::new(8 + 8, yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );
    disp.draw(
        Line::new(Coord::new(8 + 16, 16 + yoffset), Coord::new(8 + 8, yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.draw(
        Rect::new(Coord::new(48, yoffset), Coord::new(48 + 16, 16 + yoffset))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.draw(
        Circle::new(Coord::new(96, yoffset + 8), 8)
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    disp.flush().unwrap();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
