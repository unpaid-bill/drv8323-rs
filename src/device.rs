use embedded_hal as hal;
use hal::blocking::spi;
use hal::digital::v2::OutputPin;
use crate::registers;
use crate::faults::{DrvResult, DrvFault};

/// DRV8323 driver
pub struct DRV8323<SPI, CS, EN, CAL, FAULT> {
    spi: SPI,
    chip_select_pin: CS,
    enable_pin: EN,
    calibration_pin: CAL,
    nfault_pin: FAULT,
    
}

impl<SPI, CS, EN, CAL, FAULT, SpiErr, PinErr> DRV8323<SPI, CS, EN, CAL, FAULT> 
where 
    SPI: spi::Transfer<u8, Error = SpiErr> + spi::Write<u8, Error = SpiErr>,
    CS: OutputPin<Error = PinErr>,
    EN: OutputPin<Error = PinErr>,
    CAL: OutputPin<Error = PinErr>,
    FAULT: OutputPin<Error = PinErr>,
{
    /// Instantiates a new drv8323 from an SPI peripheral and four GPIO pins.
    pub fn new(spi: SPI, cs: CS, enable: EN, cal: CAL, nfault: FAULT) -> Result<Self, SpiErr>{
        let drv = DRV8323{
            spi:spi,
            chip_select_pin: cs,
            enable_pin: enable,
            calibration_pin: cal,
            nfault_pin: nfault,
        };
        Ok(drv)
    }

    pub fn enable(&mut self){
        self.enable_pin.set_high().ok();
    }
    
    pub fn disable(&mut self){
        self.enable_pin.set_low().ok();
    }

    pub fn check_faults(&mut self) -> DrvResult{
        let fs1 = registers::read_register(&mut self.spi, registers::DrvRegister::FaultStatus1);
        let fs2 = registers::read_register(&mut self.spi, registers::DrvRegister::FaultStatus2);

        let error: DrvFault;
        Ok(())
    }
}