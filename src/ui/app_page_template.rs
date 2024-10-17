use crate::ui::primary_menu::primary_menu;
use relm4;
use relm4::adw::prelude::*;
use relm4::prelude::*;
use relm4::{adw, gtk};

#[relm4::widget_template(pub)]
impl WidgetTemplate for AppPageTemplate {
    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

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
                    #[name = "container"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_top: 24,
                        set_margin_bottom: 24,
                        set_margin_start: 12,
                        set_margin_end: 12,
                    }
                }
            }
        }
    }
}
