use relm4::adw::prelude::*;
use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4::{adw, gtk};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutDialog;
    type Input = ();
    type Output = ();
    type Root = adw::AboutDialog;

    fn init_root() -> Self::Root {
        adw::AboutDialog::builder()
            .application_icon("pl.mdarocha.Secreto")
            .application_name("Secreto")
            .developer_name("Marek Darocha")
            .version(env!("VERSION"))
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/mdarocha/secreto")
            .issue_url("https://github.com/mdarocha/secreto/issues")
            .copyright("© 2024 Marek Darocha")
            .build()
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();
        widgets.present(Some(&relm4::main_application().windows()[0]));

        ComponentParts { model, widgets }
    }
}
