// use embedded_hal;
use embedded_hal as hal;


use hal::blocking::spi;
use hal::digital::v2::OutputPin;
use crate::registers;


pub struct DRV8323<SPI, EN, CAL, FAULT> {
    spi: SPI,
    enable_pin: EN,
    calibration_pin: CAL,
    nfault_pin: FAULT,
    
}

impl<SPI, EN, CAL, FAULT, E, PinError> DRV8323<SPI, EN, CAL, FAULT> 
where 
    SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
    EN: OutputPin<Error = PinError>,
    CAL: OutputPin<Error = PinError>,
    FAULT: OutputPin<Error = PinError>,
{
    pub fn new(spi: SPI, enable: EN, cal: CAL, nfault: FAULT) -> Result<Self, E>{
        let drv = DRV8323{
            spi:spi,
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
}