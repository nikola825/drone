use common::command::{CommandArgs, CommandExecutor, CommandPacket, CommandServer};
use defmt::info;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Receiver};
use zerocopy::{FromBytes, LittleEndian, U32};

use crate::storage::Store;

struct Executor {
    store: &'static Store,
}

impl CommandExecutor for Executor {
    async fn start_drone(&self, _: CommandArgs) {
        info!("Start");
    }

    async fn stop_drone(&self, _: CommandArgs) {
        info!("Stop");
    }

    async fn set_storage(&self, args: CommandArgs) {
        info!("Storage");
        let index: U32<LittleEndian> = U32::read_from(&args.data[..4]).unwrap();

        self.store
            .state
            .lock()
            .await
            .variables
            .set_variable(index.get() as usize, &args.data[4..]);
    }

    async fn send_heartbeat(&self, _: CommandArgs) {
        info!("Heartbeat")
    }
}

#[embassy_executor::task]
pub async fn commands_task(
    recv: Receiver<'static, ThreadModeRawMutex, CommandPacket, 10>,
    store: &'static Store,
) {
    info!("Commands start");
    let executor = Executor { store: store };
    loop {
        let packet = recv.receive().await;
        info!("Commands received {}", packet);
        CommandServer::execute_command(packet, &executor).await;
    }
}
