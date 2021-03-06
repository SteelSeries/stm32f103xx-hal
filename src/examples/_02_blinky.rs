//! Blinks an LED
//!
//! ```
//! 
//! #![no_std]
//! 
//! extern crate stm32f103xx_hal as hal;
//! #[macro_use(block)]
//! extern crate nb;
//! 
//! use hal::prelude::*;
//! use hal::stm32f103xx;
//! use hal::timer::Timer;
//! 
//! fn main() {
//!     let p = stm32f103xx::Peripherals::take().unwrap();
//! 
//!     let mut flash = p.FLASH.constrain();
//!     let mut rcc = p.RCC.constrain();
//! 
//!     // Try a different clock configuration
//!     // let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     let clocks = rcc.cfgr
//!         .sysclk(64.mhz())
//!         .pclk1(32.mhz())
//!         .freeze(&mut flash.acr);
//! 
//!     let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
//! 
//!     let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
//!     // Try a different timer
//!     let mut timer = Timer::tim2(p.TIM2, 1.hz(), clocks, &mut rcc.apb1);
//!     loop {
//!         block!(timer.wait()).unwrap();
//!         led.set_high();
//!         block!(timer.wait()).unwrap();
//!         led.set_low();
//!     }
//! }
//! ```
// Auto-generated. Do not modify.
