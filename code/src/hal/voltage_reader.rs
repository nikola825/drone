use core::ops::RangeInclusive;

use embassy_stm32::adc::{Adc, AnyAdcChannel};

pub struct VoltageReader<AdcType: embassy_stm32::adc::Instance> {
    pub adc: Adc<'static, AdcType>,
    pub pin: AnyAdcChannel<AdcType>,
    pub adc_range_max: u16,
    pub resistor_divider_factor: f32,
    pub acceptable_voltage_range: RangeInclusive<f32>,
    pub voltage_reference: f32,
}

impl<AdcType: embassy_stm32::adc::Instance> VoltageReader<AdcType> {
    pub fn new(pin: AnyAdcChannel<AdcType>, adc: AdcType) -> Self {
        let mut adc = Adc::new(adc);

        adc.set_averaging(embassy_stm32::adc::Averaging::Samples16);
        adc.set_sample_time(embassy_stm32::adc::SampleTime::CYCLES32_5);

        adc.set_resolution(embassy_stm32::adc::Resolution::BITS12);

        Self {
            adc,
            pin,
            adc_range_max: 4096u16,
            resistor_divider_factor: 11f32,
            acceptable_voltage_range: 0f32..=28f32,
            voltage_reference: 3.3f32,
        }
    }

    pub fn get_voltage(&mut self) -> f32 {
        let measurement = self.adc.blocking_read(&mut self.pin);

        let measured_voltage = ((measurement as f32) * self.voltage_reference)
            * self.resistor_divider_factor
            / (self.adc_range_max as f32);

        if self.acceptable_voltage_range.contains(&measured_voltage) {
            measured_voltage
        } else {
            0f32
        }
    }
}
