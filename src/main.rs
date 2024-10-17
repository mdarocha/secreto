mod password_store;
mod ui;

use crate::ui::app::{App, AppInit};
use crate::ui::primary_menu::init_primary_menu_actions;
use relm4::{main_application, RelmApp, gtk::{gio, gdk, prelude::*}, gtk};

fn init_icons() {
    gio::resources_register_include!("icons.gresource")
        .expect("Failed to register icons resources");

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/pl/mdarocha/Secreto/icons");
}

fn main() {
    // Init GTK
    gtk::init().unwrap();

    // Init GTK application
    let app = main_application();
    app.set_application_id(Some("pl.mdarocha.Secreto"));

    // Init icons
    init_icons();

    // Init actions
    let actions = init_primary_menu_actions(&app);

    // Setup and start relm app
    let app = RelmApp::from_app(app);

    app.visible_on_activate(false).run::<App>(AppInit { actions });
}
