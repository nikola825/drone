mod drone_connection;
mod gui;

use std::error::Error;

use crate::gui::Gui;

fn main() -> Result<(), Box<dyn Error>> {
    Gui::run_application()?;

    Ok(())
}
