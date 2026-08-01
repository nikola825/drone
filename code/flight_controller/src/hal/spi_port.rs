use embassy_stm32::gpio::AnyPin;
use embassy_stm32::Peri;
use embassy_stm32::{
    gpio::Output,
    mode::Async,
    spi::{self, BitOrder, Mode, Spi},
    time::Hertz,
};

pub trait SpiMaker {
    fn make_spi(
        self,
        frequency: Hertz,
        mode: Mode,
        bit_order: BitOrder,
    ) -> (Output<'static>, Spi<'static, Async>);
}

pub struct SpiPort<
    Stm32SpiInstance: embassy_stm32::spi::Instance + 'static,
    SckPin: embassy_stm32::spi::SckPin<Stm32SpiInstance> + 'static,
    MosiPin: embassy_stm32::spi::MosiPin<Stm32SpiInstance> + 'static,
    MisoPin: embassy_stm32::spi::MisoPin<Stm32SpiInstance> + 'static,
    TxDma: embassy_stm32::spi::TxDma<Stm32SpiInstance> + 'static,
    RxDma: embassy_stm32::spi::RxDma<Stm32SpiInstance> + 'static,
> {
    pub peripheral: Peri<'static, Stm32SpiInstance>,
    pub sck_pin: Peri<'static, SckPin>,
    pub mosi_pin: Peri<'static, MosiPin>,
    pub miso_pin: Peri<'static, MisoPin>,
    pub rx_dma: Peri<'static, RxDma>,
    pub tx_dma: Peri<'static, TxDma>,
    pub cs_pin: Peri<'static, AnyPin>,
}

impl<
        Stm32SpiInstance: embassy_stm32::spi::Instance + 'static,
        SckPin: embassy_stm32::spi::SckPin<Stm32SpiInstance> + 'static,
        MosiPin: embassy_stm32::spi::MosiPin<Stm32SpiInstance> + 'static,
        MisoPin: embassy_stm32::spi::MisoPin<Stm32SpiInstance> + 'static,
        TxDma: embassy_stm32::spi::TxDma<Stm32SpiInstance> + 'static,
        RxDma: embassy_stm32::spi::RxDma<Stm32SpiInstance> + 'static,
    > SpiMaker for SpiPort<Stm32SpiInstance, SckPin, MosiPin, MisoPin, TxDma, RxDma>
{
    fn make_spi(
        self,
        frequency: Hertz,
        mode: Mode,
        bit_order: BitOrder,
    ) -> (Output<'static>, Spi<'static, embassy_stm32::mode::Async>) {
        let mut spi_config = spi::Config::default();
        spi_config.frequency = frequency;
        spi_config.mode = mode;
        spi_config.bit_order = bit_order;

        (
            Output::new(
                self.cs_pin,
                embassy_stm32::gpio::Level::High,
                embassy_stm32::gpio::Speed::VeryHigh,
            ),
            spi::Spi::new(
                self.peripheral,
                self.sck_pin,
                self.mosi_pin,
                self.miso_pin,
                self.tx_dma,
                self.rx_dma,
                spi_config,
            ),
        )
    }
}
