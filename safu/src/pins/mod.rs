pub mod gpio;


pub enum Mode {
    GPIO,
    PWM,
    UART,
    ADC
}

pub fn setup_pins() {
    // Note that you cannot setup multiple functions for a
    // single pin at the same time.
    gpio::setup_gpio_pins();
}

fn enable_pin_functions(pin_no: usize, modes: Vec<Mode>) {


}