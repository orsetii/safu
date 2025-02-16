use crate::Addr;

const GPIO_IN_OFFSET: usize = 0x3fffffff;

const BANK0_BASE: Addr = Addr(0x40014000);
const QSPI_BASE: Addr = Addr(0x40018000);
const GPIO_CTRL: Addr = BANK0_BASE.wrapping_add(0x04);
const SIO_BASE: Addr = Addr(0xD0000000);
const GPIO_IN: Addr = SIO_BASE.wrapping_add(GPIO_IN_OFFSET);

// GPIO function values (FUNCSEL values in GPIO registers)
pub const GPIO_FUNC_XIP: u32  = 0b00000;  // External Flash Interface
pub const GPIO_FUNC_SPI: u32  = 0b00001;  // SPI Interface
pub const GPIO_FUNC_UART: u32 = 0b00010;  // UART Interface
pub const GPIO_FUNC_I2C: u32  = 0b00011;  // I2C Interface
pub const GPIO_FUNC_PWM: u32  = 0b00100;  // PWM Interface
pub const GPIO_FUNC_SIO: u32  = 0b00101;  // Single-cycle I/O (GPIO)
pub const GPIO_FUNC_PIO0: u32 = 0b00110;  // Programmable I/O 0
pub const GPIO_FUNC_PIO1: u32 = 0b00111;  // Programmable I/O 1
pub const GPIO_FUNC_GPCK: u32 = 0b01000;  // Clock output
pub const GPIO_FUNC_USB: u32  = 0b01001;  // USB Interface
pub const GPIO_FUNC_NULL: u32 = 0b11111;  // Special value - no function assigned

// Pad control values (drive strength)
pub const GPIO_PAD_DRIVE_2MA: u32   = 0b00;
pub const GPIO_PAD_DRIVE_4MA: u32   = 0b01;
pub const GPIO_PAD_DRIVE_8MA: u32   = 0b10;
pub const GPIO_PAD_DRIVE_12MA: u32  = 0b11;

// GPIO control register bit positions and masks
pub const GPIO_CTRL_FUNCSEL_LSB: u32 = 0;
pub const GPIO_CTRL_FUNCSEL_BITS: u32 = 0b11111;
pub const GPIO_CTRL_OUTOVER_LSB: u32 = 8;
pub const GPIO_CTRL_OUTOVER_BITS: u32 = 0b11;
pub const GPIO_CTRL_OEOVER_LSB: u32 = 12;
pub const GPIO_CTRL_OEOVER_BITS: u32 = 0b11;
pub const GPIO_CTRL_INOVER_LSB: u32 = 16;
pub const GPIO_CTRL_INOVER_BITS: u32 = 0b11;

// GPIO pad control register bit positions
pub const GPIO_PAD_DRIVE_LSB: u32 = 0;
pub const GPIO_PAD_DRIVE_BITS: u32 = 0b11;
pub const GPIO_PAD_IE_BIT: u32 = 6;      // Input Enable
pub const GPIO_PAD_OD_BIT: u32 = 7;      // Output Disable
pub const GPIO_PAD_PUE_BIT: u32 = 3;     // Pull-up Enable
pub const GPIO_PAD_PDE_BIT: u32 = 2;     // Pull-down Enable
pub const GPIO_PAD_SCHMITT_BIT: u32 = 1;  // Schmitt trigger
pub const GPIO_PAD_SLEWFAST_BIT: u32 = 0; // Slew rate control



// Each GPIO pin has to have every capability it will use, enabled 
// before it is used.
pub fn setup_gpio_pins() {

}

fn setup_pin(pin_no: usize) {
    let pin_addr = GPIO_IN.wrapping_add(8 * pin_no);
    let mut ctrl_val: u32 =  unsafe { pin_addr.read() };
    ctrl_val &= !GPIO_CTRL_FUNCSEL_BITS;
    ctrl_val |= GPIO_FUNC_SIO;
    for i in 0..3 {
        unsafe {
            // FIXME: does this shit actually work should double check
            // setup a test of this shit
            pin_addr.write_byte((ctrl_val >> 8*i) as u8);
        }
    }
}



fn write(set_high: bool, pin_no: usize) {

}