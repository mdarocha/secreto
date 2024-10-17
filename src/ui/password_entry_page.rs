use crate::password_store::{DecryptedPasswordEntry, PasswordEntry, PasswordStore};
use crate::ui::app_page_template::AppPageTemplate;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use relm4::{adw, gtk};

pub struct PasswordEntryPageInit {
    pub store: PasswordStore,
    pub entry: PasswordEntry,
}

pub struct PasswordEntryPage {
    name: String,
    password: Result<DecryptedPasswordEntry, String>,
}

// TODO make this AsyncComponent
#[relm4::component(pub)]
impl Component for PasswordEntryPage {
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
                            gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::None,

                                adw::ActionRow {
                                    #[watch]
                                    set_title: &password.password
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

    fn init(
        init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let password = init
            .store
            .decrypt(&init.entry)
            .map_err(|err| err.to_string());
        let model = PasswordEntryPage {
            name: init.entry.name.clone(),
            password,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
