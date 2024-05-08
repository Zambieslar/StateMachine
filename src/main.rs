mod constants;
mod definitions;
mod statemachine;

use std::io::Read;

use gtk4::gio::{Cancellable, FILE_ATTRIBUTE_STANDARD_SIZE};
use gtk4::ApplicationWindow;
use gtk4::{prelude::*, Application, FileDialog};

use crate::definitions::*;
use crate::statemachine::*;

fn main() {
    let app = Application::builder()
        .application_id("statemachine")
        .build();
    app.connect_activate(|app| {
        let file_dialog = FileDialog::builder().build();
        let mwindow = ApplicationWindow::builder()
            .width_request(600)
            .height_request(400)
            .resizable(false)
            .build();
        gtk4::glib::spawn_future_local(async move {
            let file = file_dialog.open_future(Some(&mwindow)).await;
            match file {
                Ok(data) => {
                    if let Ok(data) = data.read(Cancellable::NONE) {
                        let buf: u8 = Box::new(
                    }
                    let mut machine = StateMachine::new();
                }
                Err(error) => {}
            }
            //            machine.run(&file);
        });
    });
}
