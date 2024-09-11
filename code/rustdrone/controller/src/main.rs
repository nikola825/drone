use std::{error::Error, io::stdin, str::FromStr, time::Duration};

use bluer::{
    rfcomm::{Socket, SocketAddr, Stream},
    Address,
};
use common::{
    command::{CommandArgs, CommandClient, CommandPacket, CommandSender},
    storedvars::{VariableSetter, VariableSetterClient},
};
use fltk::{app, button::Button, frame::Frame, prelude::*, valuator::Dial, window::Window};
use fltk_theme::{color_themes, colors::html::Red, ColorTheme, WidgetScheme, WidgetTheme};
use gilrs::Gilrs;
use rand::Rng;
use tokio::{self, io::AsyncWriteExt};
use zerocopy::{AsBytes, LittleEndian, Unaligned, F32, U32};

struct Sender {
    pub stream: Stream,
}

impl CommandSender for Sender {
    async fn send_command(&mut self, packet: CommandPacket) {
        let mut packet = packet;
        let mut buff = [0u8; size_of::<CommandPacket>()];
        packet.checksum = 7u8;
        packet.write_to(&mut buff);
        //println!("AAAAAA {:?}", buff);
        self.stream.write_all(&buff).await.unwrap();
    }
}

async fn get_bt_stream() -> Result<Stream, Box<dyn Error>> {
    const MAX_RETRIES: i32 = 5;
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    for retry_count in 1..MAX_RETRIES {
        let mut stream =
            Stream::connect(SocketAddr::new(Address::from_str("00:22:05:00:31:68")?, 1)).await?;
        match stream.write_all(b"DUMMY").await {
            Ok(_) => {
                return Ok(stream);
            }
            Err(err) => {
                if retry_count + 1 == MAX_RETRIES {
                    return Err(Box::new(err));
                }
            }
        };
    }
    Err("FAIL".into())
}

struct VSET {
    sender: Sender,
}

impl VariableSetter for VSET {
    async fn set_variable<T: Unaligned + AsBytes>(&mut self, index: usize, value: T) {
        let index: U32<LittleEndian> = U32::from(index as u32);
        let mut data = [0u8; 12];
        index.write_to(&mut data[..4]);
        value.write_to(&mut data[4..4 + size_of::<T>()]);

        let args = CommandArgs { data: data };
        CommandClient::set_storage(args, &mut self.sender).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut stream = get_bt_stream().await?;
    /*let mut gilrs = Gilrs::new().unwrap();
    let mut selected_gamepad = None;

    for (gpid, gamepad) in gilrs.gamepads() {
        println!("{:?}", gamepad.name());
        selected_gamepad = Some(gpid);
    }


    loop {
        let event = gilrs.next_event_blocking(Some(Duration::from_millis(10)));
        if let Some(event) = event {
            if event.id == selected_gamepad.unwrap() {
                println!("{:?}", event);
            }
        }
    }*/

    /*println!("HERE1");
    
    println!("HERE2");
    stream.write_all(b"AA").await.unwrap();
    println!("HERE3");
    let mut sender = Sender { stream: stream };
    let mut vset = VSET { sender: sender };

    

    loop {
        let aa = CommandArgs {
            data: [
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8,
            ],
        };
        CommandClient::send_heartbeat(aa, &mut vset.sender).await;
        VariableSetterClient::Yaw_Kp(rand::thread_rng().gen(), &mut vset).await;

    }*/

    let app = app::App::default();
    
    /*let ws = WidgetScheme::new(fltk_theme::SchemeType::Aqua);
    ws.apply();
    let ct = ColorTheme::new(color_themes::BLACK_THEME);
    ct.apply();*/
    let wt = WidgetTheme::new(fltk_theme::ThemeType::HighContrast);
    wt.apply();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    let mut dial = Dial::new(10, 10, 50, 50, "AA");
    dial.set_color(Red);
    dial.set_frame(fltk::enums::FrameType::EmbossedBox);
    wind.end();
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!")); // the closure capture is mutable borrow to our button
    app.run().unwrap();

    Ok(())
}
