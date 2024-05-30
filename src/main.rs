#![allow(dead_code, unused_imports)]

mod conditions;
mod constants;
mod definitions;
mod statemachine;
mod traits;

use std::io::Read;

use gtk4::gio::{Cancellable, FILE_ATTRIBUTE_STANDARD_SIZE};
use gtk4::ApplicationWindow;
use gtk4::{prelude::*, Application, FileDialog};

use crate::statemachine::*;

use crate::{constants::*, definitions::*, traits::*};

fn main() {
    let app = Application::builder()
        .application_id("com.zambiebam.statemachine")
        .build();
    app.connect_activate(|app| {
        let file_dialog = FileDialog::builder().build();
        let mwindow = ApplicationWindow::builder()
            .width_request(600)
            .height_request(400)
            .resizable(false)
            .build();
        mwindow.present();
        gtk4::glib::spawn_future_local(async move {
            let file = file_dialog.open_future(Some(&mwindow)).await;
            match file {
                Ok(data) => {
                    if let Ok(data) = data.read(Cancellable::NONE) {
                        let size = data
                            .query_info(FILE_ATTRIBUTE_STANDARD_SIZE, Cancellable::NONE)
                            .expect("Unable to read attribute")
                            .size();
                        let mut buf = Box::new(Vec::<u8>::new());
                        data.into_read().bytes().map(|x| buf.push(x.unwrap()));
                        let mut machine = StateMachine::new();
                        machine.run(&buf);
                    }
                }
                Err(error) => {}
            }
            //            machine.run(&file);
        });
    });
    app.run();
}
