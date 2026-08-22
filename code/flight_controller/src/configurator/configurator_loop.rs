use common::configurator_protocol::messages::{FcPhase, FcStatus};
use defmt::info;
use embassy_time::{Duration, Instant, Ticker};

use crate::{
    configurator::motor_setting_state_machine::MotorSettingStateMachine,
    esc::{motor_control::DshotCommand, EscMotorSet},
    hal::{Leds, ESC_COUNT},
    shared_state::SharedState,
};

pub async fn configurator_loop(
    mut motors: EscMotorSet,
    shared_state: &'static SharedState,
    mut leds: Leds,
    fc_phase_at_exit: FcPhase,
) -> ! {
    const PID_PERIOD_US: u64 = 1005;
    let mut ticker = Ticker::every(Duration::from_micros(PID_PERIOD_US));

    let mut print_counter = 0;

    // If previous phase was Disarmed, the config loop was entered via normal request
    // Any other phase indicates an init failure and should be emitted to configurator
    let phase_to_broadcast = match fc_phase_at_exit {
        FcPhase::Disarmed => FcPhase::Config,
        _ => fc_phase_at_exit,
    };

    leds.green_off();
    leds.blue_off();
    leds.yellow_on();

    let mut total_duration = 0f32;
    let mut inner_duration = 0f32;
    let mut previous_t1 = Instant::now();
    let mut max_measured_period = 0;
    let mut min_measured_period = 10000;

    let mut motor_setting: Option<MotorSettingStateMachine> = None;

    loop {
        let t1 = Instant::now();

        let t2 = Instant::now();
        print_counter += 1;

        let mut motor_commands: [DshotCommand; ESC_COUNT] =
            [DshotCommand::DSHOT_CMD_STOP; ESC_COUNT];

        if let Some(setting) = motor_setting.as_mut() {
            if let Some(command) = setting.advance() {
                motor_commands[setting.motor_index() as usize] = command;
            } else {
                motor_setting = None;
            }
        } else {
            motor_setting = shared_state
                .query_motor_direction_setting()
                .map(|direction_setting| direction_setting.into());
        }

        motors.multi_send_command(motor_commands).await;

        let t3 = Instant::now();

        total_duration = (t3 - t1).as_micros() as f32 * 0.5 + total_duration * 0.5;
        inner_duration = (t3 - t2).as_micros() as f32 * 0.5 + inner_duration * 0.5;
        let measured_period: u64 = (t1 - previous_t1).as_micros();
        if measured_period > max_measured_period {
            max_measured_period = measured_period;
        }
        if measured_period < min_measured_period {
            min_measured_period = measured_period;
        }

        if print_counter > 1000 * (2000 / PID_PERIOD_US) {
            print_counter = 0;
            info!(
                "TICK {} {} {} {}",
                total_duration, inner_duration, min_measured_period, max_measured_period,
            );
            shared_state.publish_status(FcStatus {
                inner_duration: inner_duration.into(),
                total_duration: total_duration.into(),
                min_measured_period: min_measured_period.into(),
                max_measured_period: max_measured_period.into(),
                fc_phase: phase_to_broadcast,
                valid: true,
            });
            max_measured_period = 0;
            min_measured_period = 10000;
        }
        previous_t1 = t1;
        ticker.next().await;
    }
}
