use crate::password_store::{PasswordEntry, PasswordItem, PasswordStore};
use crate::ui::password_item_view::PasswordItemViewOutputs;
use crate::ui::primary_menu::primary_menu;
use relm4::adw::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use relm4::{adw, gtk};

pub struct PasswordListPageInit {
    pub store: PasswordStore,
    pub subdir: String,
}

pub struct PasswordListPage {
    store: PasswordStore,
    subdir: String,
    passwords: FactoryVecDeque<PasswordItem>,
}

#[derive(Debug)]
pub enum PasswordListInputs {
    LoadPasswordList,
    PasswordItemClicked(usize),
}

#[derive(Debug)]
pub enum PasswordListOutputs {
    OpenSubdir(String),
    OpenEntry(PasswordEntry),
}

#[derive(Debug)]
pub enum PasswordListCommands {
    PasswordListLoaded(Vec<PasswordItem>),
}

#[relm4::component(pub)]
impl Component for PasswordListPage {
    type Init = PasswordListPageInit;
    type Input = PasswordListInputs;
    type Output = PasswordListOutputs;
    type CommandOutput = PasswordListCommands;

    view! {
        adw::NavigationPage {
            #[watch]
            set_title: if &model.subdir == "." {
                "Secreto"
            } else {
                &model.subdir
            },

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Raised,

                add_top_bar = &adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu())
                    }
                },

                gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hscrollbar_policy: gtk::PolicyType::Never,

                    adw::Clamp {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_margin_top: 24,
                            set_margin_bottom: 24,
                            set_margin_start: 12,
                            set_margin_end: 12,

                            #[local_ref]
                            password_list_widget -> gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::None
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let passwords = FactoryVecDeque::<PasswordItem>::builder()
            .launch_default()
            .forward(sender.input_sender(), |output| match output {
                PasswordItemViewOutputs::Clicked(index) => {
                    PasswordListInputs::PasswordItemClicked(index)
                }
            });

        let model = Self {
            store: init.store.clone(),
            subdir: init.subdir,
            passwords,
        };

        let password_list_widget = model.passwords.widget();
        let widgets = view_output!();

        sender.input(PasswordListInputs::LoadPasswordList);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PasswordListInputs::LoadPasswordList => {
                let store = self.store.clone();
                let subdir = self.subdir.clone();
                sender.spawn_oneshot_command(move || {
                    let password_list = {
                        match store.list(&subdir) {
                            Ok(passwords) => passwords,
                            Err(_) => todo!(),
                        }
                    };
                    PasswordListCommands::PasswordListLoaded(password_list)
                });
            }
            PasswordListInputs::PasswordItemClicked(index) => {
                let item = self.passwords.get(index);
                match item {
                    Some(PasswordItem::Directory(directory)) => {
                        sender
                            .output(PasswordListOutputs::OpenSubdir(directory.path.clone()))
                            .expect("No receivers!");
                    }
                    Some(PasswordItem::Entry(entry)) => {
                        sender
                            .output(PasswordListOutputs::OpenEntry(entry.clone()))
                            .expect("No receivers!");
                    }
                    None => {}
                }
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            PasswordListCommands::PasswordListLoaded(password_list) => {
                let mut guard = self.passwords.guard();
                for password in password_list {
                    guard.push_back(password);
                }
            }
        }
    }
}
