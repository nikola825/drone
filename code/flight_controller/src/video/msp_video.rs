// We do a lot of byte array manipulation here. Forbid indexing on things of uknown size
#![forbid(clippy::indexing_slicing)]

mod char_map_hdzero_inav;

use crate::{
    hal::OptionalOutput,
    msp::{
        msp_displayport::{clear_display, draw_display, set_resolution, write_string_to_screen},
        protocol::{transmit_fc_variant_inav, transmit_status, transmit_sticks},
    },
    video::VideoController,
};
#[allow(unused_imports)]
pub use char_map_hdzero_inav::OSDSymbol;
use common::msp_protocol::displayport::HDZeroResolution;

use crate::{hal::UartMaker, shared_state::CommandState};
use embassy_stm32::{
    mode::Async,
    usart::{Parity, StopBits, UartRx, UartTx},
};

const LEFT_COLUMN: u8 = 0;
const RIGHT_COLUMN: u8 = 44;

#[allow(dead_code)]
pub struct MspVideoController {
    tx: UartTx<'static, Async>,
    vtx_power_toggle: OptionalOutput,
    left_column_tracker: ColumnPositionTracker,
    right_column_tracker: ColumnPositionTracker,
}

#[allow(dead_code)]
impl VideoController for MspVideoController {
    fn new(msp_uart: impl UartMaker, vtx_power_toggle: OptionalOutput) -> Self {
        let (_, tx) = make_msp_uart_pair(msp_uart);

        Self {
            tx,
            vtx_power_toggle,
            left_column_tracker: ColumnPositionTracker::new(LEFT_COLUMN, 2),
            right_column_tracker: ColumnPositionTracker::new(RIGHT_COLUMN, 2),
        }
    }

    async fn init(&mut self) {
        self.vtx_power_toggle.set_high();
    }

    async fn enter_low_power_mode(&mut self) {
        self.vtx_power_toggle.set_low();
    }

    async fn enter_full_power_mode(&mut self) {
        self.vtx_power_toggle.set_high();
    }

    async fn transmit_boilerplate(&mut self) {
        let _ = set_resolution(&mut self.tx, HDZeroResolution::HD_5018).await;
        let _ = transmit_fc_variant_inav(&mut self.tx).await; // Pretend to be INAV - I like the character map
    }

    async fn transmit_sticks(&mut self, sticks: &CommandState) {
        let _ = transmit_sticks(&mut self.tx, sticks).await;
    }

    async fn transmit_arming_state(&mut self, armed: bool) {
        let _ = transmit_status(&mut self.tx, armed).await;
    }

    async fn begin_refresh(&mut self) {
        self.left_column_tracker = ColumnPositionTracker::new(LEFT_COLUMN, 2);
        self.right_column_tracker = ColumnPositionTracker::new(RIGHT_COLUMN, 2);
        let _ = clear_display(&mut self.tx).await;
    }

    async fn add_string_to_left_column<const STRING_LEN: usize>(
        &mut self,
        data: [u8; STRING_LEN],
    ) -> Result<(), embassy_stm32::usart::Error> {
        let col = self.left_column_tracker.column;
        let row = self.left_column_tracker.get_next_row();

        write_string_to_screen(&mut self.tx, row, col, data).await
    }

    async fn add_string_to_right_column<const STRING_LEN: usize>(
        &mut self,
        data: [u8; STRING_LEN],
    ) -> Result<(), embassy_stm32::usart::Error> {
        let col = self.right_column_tracker.column;
        let row = self.right_column_tracker.get_next_row();

        write_string_to_screen(&mut self.tx, row, col, data).await
    }

    async fn end_refresh(&mut self) {
        let _ = draw_display(&mut self.tx).await;
    }
}

struct ColumnPositionTracker {
    current_row: u8,
    column: u8,
}

impl ColumnPositionTracker {
    fn new(column: u8, initial_row: u8) -> Self {
        Self {
            current_row: initial_row,
            column,
        }
    }

    fn get_next_row(&mut self) -> u8 {
        let row = self.current_row;
        self.current_row += 1;
        row
    }
}

fn make_msp_uart_pair(
    uart_getter: impl UartMaker,
) -> (UartRx<'static, Async>, UartTx<'static, Async>) {
    let (rx, tx) = uart_getter.make_uart(115200, Parity::ParityNone, StopBits::STOP1, false);

    (rx, tx)
}
