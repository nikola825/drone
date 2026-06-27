use std::{default, io::{Read, Write}, time::Duration};

use common::msp_protocol::{messages::ApiVersionMessage, protocol::MSPMessage};
use fltk::{app, button::Button, frame::Frame, group::{Flex, Grid}, prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt}, window::Window};
use zerocopy::{IntoBytes, TryFromBytes};


mod gui;
mod drone_connection;

fn main() {
    let mut g = gui::Gui::new();
    g.run();
}
