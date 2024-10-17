mod password_store;
mod ui;

use crate::ui::app;
use relm4::{prelude::*, gtk::{gio, gdk}, gtk};

fn init_icons() {
    gio::resources_register_include!("icons.gresource")
        .expect("Failed to register icons resources");

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/pl/mdarocha/Secreto/icons");
}

fn main() {
    let app = RelmApp::new("pl.mdarocha.Secreto");

    init_icons();

    app.run::<app::App>(());
}
