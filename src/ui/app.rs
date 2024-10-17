use crate::password_store::{PasswordEntry, PasswordStore};
use crate::ui::password_entry_page::{PasswordEntryPage, PasswordEntryPageInit};
use crate::ui::password_list_page::{PasswordListOutputs, PasswordListPage, PasswordListPageInit};
use crate::ui::primary_menu::WindowActionGroup;
use relm4::actions::RelmActionGroup;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use std::collections::VecDeque;
use std::env;

pub enum Pages {
    PasswordListPage(Controller<PasswordListPage>),
    PasswordEntryPage(Controller<PasswordEntryPage>),
}

pub struct AppInit {
    pub actions: RelmActionGroup<WindowActionGroup>,
}

pub struct App {
    store: PasswordStore, // TODO move PasswordStore to a worker to avoid clones
    password_views: VecDeque<Pages>,
}

#[derive(Debug)]
pub enum AppInputs {
    OpenPasswordsSubdir(String),
    OpenPasswordEntry(PasswordEntry),
    PasswordViewClosed,
}

#[relm4::component(pub)]
impl Component for App {
    type Init = AppInit;
    type Input = AppInputs;
    type Output = ();
    type CommandOutput = ();

    view! {
        main_window = adw::ApplicationWindow {
            set_visible: true,
            set_default_size: (800, 500),

            #[name = "navigation_view"]
            adw::NavigationView {
                connect_popped[sender] => move |_, _| sender.input(AppInputs::PasswordViewClosed)
            }
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // TODO allow setting the dir from GUI settings
        let store_dir = env::var("PASSWORD_STORE_DIR").unwrap();

        let model = App {
            store: PasswordStore::new(&store_dir),
            password_views: VecDeque::new(),
        };

        let widgets = view_output!();
        init.actions.register_for_widget(&widgets.main_window);

        sender.input(AppInputs::OpenPasswordsSubdir(String::from(".")));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInputs::OpenPasswordsSubdir(subdir) => {
                let controller = PasswordListPage::builder()
                    .launch(PasswordListPageInit {
                        store: self.store.clone(),
                        subdir,
                    })
                    .forward(sender.input_sender(), |message| match message {
                        PasswordListOutputs::OpenSubdir(subdir) => {
                            AppInputs::OpenPasswordsSubdir(subdir)
                        }
                        PasswordListOutputs::OpenEntry(entry) => {
                            AppInputs::OpenPasswordEntry(entry)
                        }
                    });

                widgets.navigation_view.push(controller.widget());
                self.password_views
                    .push_back(Pages::PasswordListPage(controller));
            }
            AppInputs::OpenPasswordEntry(entry) => {
                let controller = PasswordEntryPage::builder()
                    .launch(PasswordEntryPageInit {
                        store: self.store.clone(),
                        entry,
                    })
                    .forward(sender.input_sender(), |message| match message {
                        _ => todo!(),
                    });

                widgets.navigation_view.push(controller.widget());
                self.password_views
                    .push_back(Pages::PasswordEntryPage(controller));
            }
            AppInputs::PasswordViewClosed => {
                self.password_views.pop_back();
            }
        };
    }
}
