mod password_store;
mod ui;

use crate::ui::app;
use relm4::prelude::*;

fn main() {
    let app = RelmApp::new(env!("APP_ID"));
    app.run::<app::App>(());
}
