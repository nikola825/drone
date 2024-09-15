use defmt::info;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[derive(AsBytes, FromBytes, FromZeroes, Unaligned, Default)]
#[repr(C)]
pub struct CommandPacket {
    pub start: u8,
    pub id: u8,
    pub args: CommandArgs,
    pub checksum: u8,
    pub end: u8,
}

impl CommandPacket {
    pub fn is_valid(&self) -> bool {
        return self.start == 0x42 && self.end == 0x24;
    }
}

#[derive(AsBytes, FromBytes, FromZeroes, Unaligned, Default)]
#[repr(C)]
pub struct CommandArgs {
    pub data: [u8; 12],
}

impl defmt::Format for CommandPacket {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "CommandPacket {}", self.id);
    }
}

pub trait CommandSender {
    async fn send_command(&mut self, packet: CommandPacket);
}

macro_rules! define_command_ids {
    ($command:ident, $($commands:ident),*)=>{
        mod command_ids {
        pub const $command : u8 = (0);
        define_command_ids!((1), $($commands),*);
        }
    };
    ($cnt: expr, $command:ident)=>{
        pub const $command : u8 = $cnt;
    };
    ($cnt: expr, $command:ident, $($commands:ident),*)=>{
        pub const $command : u8 = ($cnt);
        define_command_ids!((($cnt)+1), $($commands),*);
    };
}

macro_rules! define_command_executor_trait {
    ($($commands:ident),*)=>{
        pub trait CommandExecutor {
            $(
                async fn $commands (&self, args: CommandArgs);
            )*
        }
    };
}

macro_rules! define_command_server {
    ($($commands:ident),*)=>{
        pub struct CommandServer {}

        impl CommandServer {
            pub async fn execute_command(packet: CommandPacket, executor: &impl CommandExecutor) {
                match packet.id{
                    $(
                        command_ids:: $commands =>{
                            executor. $commands (packet.args).await;
                        }
                    )*
                    _=>{
                        info!("Uknown command {}", packet.id);
                    }
                }
            }
        }
    };
}

macro_rules! define_command_client {
    ($($commands:ident),*)=>{
        pub struct CommandClient {}

        impl CommandClient {
            $(
                pub async fn $commands (args: CommandArgs, sender: &mut impl CommandSender){
                    let packet = CommandPacket {
                        id: command_ids:: $commands,
                        args: args,
                        start: 0x42u8,
                        end: 0x24u8,
                        checksum: 0u8
                    };
                    sender.send_command(packet).await;
                }
            )*
        }
    };
}

macro_rules! define_command_list {
    ($($commands:ident),*)=>{
        define_command_ids!($($commands),*);
        define_command_executor_trait!($($commands),*);
        define_command_server!($($commands),*);
        define_command_client!($($commands),*);
    };
}

define_command_list!(start_drone, stop_drone, set_storage, send_heartbeat);
