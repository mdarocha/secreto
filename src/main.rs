mod password_store;
mod ui;

use crate::ui::app;
use relm4::prelude::*;

fn main() {
    let app = RelmApp::new("pl.mdarocha.Secreto");
    app.run::<app::App>(());
}
