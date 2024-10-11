use crate::ui::password_list_page::{PasswordListOutputs, PasswordListPage, PasswordListPageInit};
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use std::collections::VecDeque;
use std::env;

pub struct App {
    store_dir: String,
    password_list_views: VecDeque<Controller<PasswordListPage>>,
}

#[derive(Debug)]
pub enum AppInputs {
    OpenPasswordsSubdir(String),
    SubdirClosed,
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppInputs;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::ApplicationWindow {
            set_default_size: (800, 500),

            #[name = "navigation_view"]
            adw::NavigationView {
                connect_popped[sender] => move |_, _| sender.input(AppInputs::SubdirClosed)
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // TODO allow setting the dir from GUI settings
        let store_dir = env::var("PASSWORD_STORE_DIR").unwrap();

        let model = App {
            store_dir,
            password_list_views: VecDeque::new(),
        };
        let widgets = view_output!();

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
                        store_dir: self.store_dir.clone(),
                        subdir,
                    })
                    .forward(sender.input_sender(), |message| match message {
                        PasswordListOutputs::OpenSubdir(subdir) => {
                            AppInputs::OpenPasswordsSubdir(subdir)
                        }
                    });

                widgets.navigation_view.push(controller.widget());
                self.password_list_views.push_back(controller);
            }
            AppInputs::SubdirClosed => {
                self.password_list_views.pop_back();
            }
        };
    }
}
