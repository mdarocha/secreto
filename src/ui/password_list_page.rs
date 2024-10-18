use crate::password_store::{PasswordEntry, PasswordItem, PasswordStore};
use crate::ui::app_page_template::AppPageTemplate;
use crate::ui::password_item_view::PasswordItemViewOutputs;
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

            #[template]
            AppPageTemplate {
                #[template_child]
                container {
                    #[local_ref]
                    password_list_widget -> gtk::ListBox {
                        add_css_class: "boxed-list",
                        set_selection_mode: gtk::SelectionMode::None
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
                {
                    let mut guard = self.passwords.guard();
                    for password in password_list {
                        guard.push_back(password);
                    }
                }
                // focus on the first item
                if let Some(window) = relm4::main_application().active_window() {
                    if let Some(root) = window.root() {
                        let listbox = self.passwords.widget();
                        if let Some(first_row) = listbox.first_child() {
                            root.set_focus(Some(&first_row));
                        }
                    }
                }
            }
        }
    }
}
