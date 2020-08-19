use embedded_hal as hal;
use hal::blocking::spi;

enum DrvRegister{

    FaultStatus1 = 0x00,
    FaultStatus2 = 0x01,
    DriverControl = 0x02,
    GateDriveHS = 0x03,
    GateDriveLS = 0x04,
    OcpControl = 0x05,
    CsaControl = 0x06,
}

impl DrvRegister{
    pub fn addr(self) -> u8 {
        self as u8
    }
}

#[allow(non_camel_case_types)]
enum FaultStatus1 {
    Fault   = 0b0000_0100_0000_0000,
    VDS_OCP = 0b0000_0010_0000_0000,
    GDF     = 0b0000_0001_0000_0000,
    UVLO    = 0b0000_0000_1000_0000,
    OTSD    = 0b0000_0000_0100_0000,
    VDS_HA  = 0b0000_0000_0010_0000,
    VDS_LA  = 0b0000_0000_0001_0000,
    VDS_HB  = 0b0000_0000_0000_1000,
    VDS_LB  = 0b0000_0000_0000_0100,
    VDS_HC  = 0b0000_0000_0000_0010,
    VDS_LC  = 0b0000_0000_0000_0001,
}

impl FaultStatus1 {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

#[allow(non_camel_case_types)]
enum FaultStatus2 {
    SA_OC  = 0b0000_0100_0000_0000,
    SB_OC  = 0b0000_0010_0000_0000,
    SC_OC  = 0b0000_0001_0000_0000,
    OTW    = 0b0000_0000_1000_0000,
    CPUV   = 0b0000_0000_0100_0000,
    VGS_HA = 0b0000_0000_0010_0000,
    VGS_LA = 0b0000_0000_0001_0000,
    VGS_HB = 0b0000_0000_0000_1000,
    VGS_LB = 0b0000_0000_0000_0100,
    VGS_HC = 0b0000_0000_0000_0010,
    VGS_LC = 0b0000_0000_0000_0001,
}

impl FaultStatus2 {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

#[allow(non_camel_case_types)]
enum DriveControl {
    // Reserved = 0b0000_0010_0000_0000,
    DIS_CPUV = 0b0000_0010_0000_0000,
    DIS_GDF  = 0b0000_0001_0000_0000,
    OTW_REP  = 0b0000_0000_1000_0000,
    PWM_MODE = 0b0000_0000_0110_0000,
    PWM_COM  = 0b0000_0000_0001_0000,
    PWM_DIR  = 0b0000_0000_0000_1000,
    COAST    = 0b0000_0000_0000_0100,
    BRAKE    = 0b0000_0000_0000_0010,
    CLR_FLT  = 0b0000_0000_0000_0000,
}

impl DriveControl {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

#[allow(non_camel_case_types)]
enum GateDriveHS {
    LOCK       = 0b0000_0111_0000_0000,
    IDRIVEP_HS = 0b0000_0000_1111_0000,
    IDRIVEN_HS = 0b0000_0000_0000_1111,
}

impl GateDriveHS {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

#[allow(non_camel_case_types)]
enum GateDriveLS {
    CBC        = 0b0000_0100_0000_0000,
    TDRIVE     = 0b0000_0011_0000_0000,
    IDRIVEP_LS = 0b0000_0000_1111_0000,
    IDRIVEN_LS = 0b0000_0000_0000_1111,
}

impl GateDriveLS {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

#[allow(non_camel_case_types)]
enum OCPControl {
    TRETRY    = 0b0000_0100_0000_0000,
    DEAD_TIME = 0b0000_0011_0000_0000,
    OCP_MODE  = 0b0000_0000_1100_0000,
    OCP_DEG   = 0b0000_0000_0011_0000,
    VDS_LVL   = 0b0000_0000_0000_1111,
}

impl OCPControl {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

#[allow(non_camel_case_types)]
enum CSAControl {
    CSA_FET   = 0b0000_0100_0000_0000,
    VREF_DIV  = 0b0000_0010_0000_0000,
    LS_REF    = 0b0000_0001_0000_0000,
    CSA_GAIN  = 0b0000_0000_1100_0000,
    DIS_SEN   = 0b0000_0000_0010_0000,
    CSA_CAL_A = 0b0000_0000_0001_0000,
    CSA_CAL_B = 0b0000_0000_0000_1000,
    CSA_CAL_C = 0b0000_0000_0000_0100,
    SEN_LVL   = 0b0000_0000_0000_0011,
}

impl CSAControl {
    fn bitmask(self) -> u16 {
        self as u16
    }
}

enum PWM_Mode {
    SixPin,
    ThreePin,
    OnePin,
}

fn write_register<SPI, E>(
    spi: &mut SPI,
    reg: DrvRegister,
    data: u16,
) -> Result<(), E>
where
    SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
{
    // no implemented!!!!
    let mut transfer_buffer: [u8; 4];
    transfer_buffer = [0, 0, 0, 0];

    // replaces contents of transfer_buffer 
    // with recieved data as it comes in
    spi.transfer(
        &mut transfer_buffer,
    )?;

    Ok(())
}

fn read_register<SPI, E>(
    spi: &mut SPI,
    reg: DrvRegister,
    data: u16,
) -> Result<(), E>
where
    SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
{
    // no implemented!!!!
    let mut transfer_buffer: [u8; 4];
    transfer_buffer = [0, 0, 0, 0];

    // replaces contents of transfer_buffer 
    // with recieved data as it comes in
    spi.transfer(
        &mut transfer_buffer,
    )?;

    Ok(())
}