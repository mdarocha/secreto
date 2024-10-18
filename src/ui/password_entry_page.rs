use crate::password_store::{DecryptedPasswordEntry, PasswordEntry, PasswordStore};
use crate::ui::app_page_template::AppPageTemplate;
use relm4::adw::prelude::*;
use relm4::loading_widgets::LoadingWidgets;
use relm4::prelude::*;
use relm4::tokio::task::spawn_blocking;
use relm4::view;
use relm4::{adw, gtk};

pub struct PasswordEntryPageInit {
    pub store: PasswordStore,
    pub entry: PasswordEntry,
}

pub struct PasswordEntryPage {
    name: String,
    password: Result<DecryptedPasswordEntry, String>,
}

#[relm4::component(pub, async)]
impl AsyncComponent for PasswordEntryPage {
    type Init = PasswordEntryPageInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            #[watch]
            set_title: &model.name,

            #[template]
            AppPageTemplate {
                #[template_child]
                container {
                    match &model.password {
                        Ok(password) =>
                            adw::PreferencesGroup {
                                adw::PasswordEntryRow {
                                    set_title: "Password",
                                    #[watch]
                                    set_text: &password.password,

                                    add_suffix: copy_button = &gtk::Button {
                                        add_css_class: "flat",
                                        set_valign: gtk::Align::Center,

                                        set_icon_name: "edit-copy-symbolic",

                                        connect_clicked[_sender] => move |_| {
                                            println!("copying password");
                                        }
                                    }
                                }
                            },
                        Err(err) =>
                            gtk::Label {
                                #[watch]
                                set_text: &err
                            }
                    }
                }
            }
        }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local]
            root {
                set_title: "Decrypting...",

                #[template]
                AppPageTemplate {
                    #[template_child]
                    container {
                        #[name(spinner)]
                        gtk::Spinner {
                            start: (),
                            set_halign: gtk::Align::Center,
                            set_size_request: (32, 32)
                        }
                    }
                }
            }
        }

        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        init: Self::Init,
        _root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let password = {
            let store = init.store.clone();
            let entry = init.entry.clone();
            spawn_blocking(move || store.decrypt(&entry).map_err(|err| err.to_string()))
                .await
                .expect("Failed decrypt task!")
        };

        let model = PasswordEntryPage {
            name: init.entry.name.clone(),
            password,
        };

        let widgets = view_output!();

        // focus the copy button at init
        if let Some(window) = relm4::main_application().active_window() {
            if let Some(root) = window.root() {
                root.set_focus(Some(&widgets.copy_button));
                widgets.copy_button.grab_focus();
            }
        }

        AsyncComponentParts { model, widgets }
    }
}
