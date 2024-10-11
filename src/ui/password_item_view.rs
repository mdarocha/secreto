use crate::password_store::PasswordItem;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use relm4::{adw, gtk, prelude::FactoryComponent};

#[derive(Debug)]
pub enum PasswordItemViewOutputs {
    Clicked(usize),
}

#[relm4::factory(pub)]
impl FactoryComponent for PasswordItem {
    type Init = PasswordItem;
    type Input = ();
    type Output = PasswordItemViewOutputs;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow::builder().activatable(true).selectable(false).build() {
            #[watch]
            set_title: match self {
                PasswordItem::Directory(entry) => &entry.name,
                PasswordItem::Entry(entry) => &entry.name
            },

            set_subtitle: match self {
                PasswordItem::Directory(entry) => &entry.path,
                PasswordItem::Entry(entry) => &entry.path
            },

            add_prefix: &gtk::Image::from_icon_name(match self {
                PasswordItem::Directory(_) => "folder-symbolic",
                PasswordItem::Entry(_) => "key3-symbolic" // TODO add custom icon
            }),

            connect_activated[sender, index] => move |_| { sender.output(PasswordItemViewOutputs::Clicked(index.current_index())).expect("No receivers!"); }
        }
    }

    fn init_model(entry: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        entry
    }
}
