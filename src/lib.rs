//! DRV8323XS gate driver compatible with [embedded-hal]
//! 
//! ## Device
//! 
//! The Texas Instruments DRV8323 is a 65-V max 3-phase smart gate driver with current shunt amplifiers.
//! 
//! More info can be found on the [product page] and in the [datasheet]
//!
//! [product page]: https://www.ti.com/product/DRV8323
//! [datasheet]: https://www.ti.com/lit/ds/symlink/drv8323.pdf
//! 
//! ## Usage
//! ```
//! let spi = spi::Spi::spi1(dp.SPI1, spi_pins, &mut afio.mapr, spi_mode, 1.hz(), clocks, &mut rcc.apb2);
//! let enable_pin = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
//! let cal_pin = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
//! let nfault_pin = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
//! 
//! let drv = DRV8323::new(spi, cs, enable, cal, nfault)?;
//! ```
//! 
//! [embedded-hal]: https://crates.io/crates/embedded-hal

#![no_std]

mod device;
mod registers;
mod faults;
pub use device::DRV8323;
pub use faults::{DrvResult, DrvFault, Drv8323Error};
