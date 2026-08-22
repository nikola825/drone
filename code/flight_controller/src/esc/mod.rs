use crate::dshot::DshotDma;
use crate::hal::{DSHOT_DMA, DSHOT_TIMER, ESC_COUNT};
use embassy_stm32::pac::gpio::Gpio;
use embassy_stm32::{
    gpio::{AnyPin, Flex, Level, Pin},
    Peri,
};

pub mod motor_control;
pub mod serial;

pub struct EscMotor {
    flex: Flex<'static>,
}

pub struct EscMotorSet {
    pins: [u8; ESC_COUNT],
    motors: [EscMotor; ESC_COUNT],
    dshot_dma: DshotDma,
}

pub struct FourWayMotorSet {
    pub motors: [EscMotor; ESC_COUNT],
}

impl EscMotorSet {
    pub fn new(
        pins: [Peri<'static, AnyPin>; ESC_COUNT],
        dshot_dma: Peri<'static, DSHOT_DMA>,
        dshot_timer: Peri<'static, DSHOT_TIMER>,
        dshot_gpio_port: Gpio,
    ) -> Self {
        let pin_numbers: [u8; ESC_COUNT] = pins.each_ref().map(|x| x.pin());
        let motors = pins.map(|x| EscMotor::new(x));

        let mut motor_set = EscMotorSet {
            pins: pin_numbers,
            motors,
            dshot_dma: DshotDma::new(dshot_timer, dshot_dma, dshot_gpio_port, &pin_numbers),
        };

        motor_set.enter_dshot_mode();

        motor_set
    }

    fn enter_dshot_mode(&mut self) {
        self.motors.iter_mut().for_each(|x| {
            x.enter_dshot_mode();
        });
        self.dshot_dma.init();
    }

    fn enter_serial_mode(&mut self) {
        self.motors.iter_mut().for_each(|x| {
            x.enter_serial_mode();
        });
        self.dshot_dma.stop();
    }

    pub fn into_four_way(mut self) -> FourWayMotorSet {
        self.enter_serial_mode();

        FourWayMotorSet {
            motors: self.motors,
        }
    }
}

impl EscMotor {
    pub fn new(pin: Peri<'static, AnyPin>) -> Self {
        let flex = Flex::new(pin);

        let mut motor = EscMotor { flex };

        motor.enter_dshot_mode();

        motor
    }

    fn enter_dshot_mode(&mut self) {
        self.flex
            .set_as_output(embassy_stm32::gpio::Speed::VeryHigh);
        self.flex.set_level(Level::Low);
    }

    fn enter_serial_mode(&mut self) {
        self.flex.set_as_input_output_pull(
            embassy_stm32::gpio::Speed::VeryHigh,
            embassy_stm32::gpio::Pull::Up,
        );
        self.flex.set_high();
    }
}
