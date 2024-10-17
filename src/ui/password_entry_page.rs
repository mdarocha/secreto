use crate::password_store::{DecryptedPasswordEntry, PasswordEntry, PasswordStore};
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

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Raised,

                add_top_bar = &adw::HeaderBar { },

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
