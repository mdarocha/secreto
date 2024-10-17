use crate::ui::about_dialog::AboutDialog;
use relm4;
use relm4::actions::AccelsPlus;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::gtk;
use relm4::gtk::gio::Menu;
use relm4::gtk::prelude::ApplicationExt;
use relm4::prelude::*;

relm4::new_action_group!(pub WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(HelpAction, WindowActionGroup, "help");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");
relm4::new_stateless_action!(QuitAction, WindowActionGroup, "quit");

pub fn primary_menu() -> Menu {
    relm4::menu! {
        primary_menu: {
            section! {
                "Preferences" => PreferencesAction,
            },
            section! {
                "Keyboard Shortcuts" => ShortcutsAction,
                "Help" => HelpAction,
                "About Secreto" => AboutAction
            }
        }
    }

    primary_menu
}

pub fn init_primary_menu<W>(widget: W)
where
    W: AsRef<gtk::Widget>,
{
    let actions = {
        let mut actions = RelmActionGroup::<WindowActionGroup>::new();

        actions.add_action(RelmAction::<PreferencesAction>::new_stateless(move |_| {
            todo!();
        }));

        actions.add_action(RelmAction::<ShortcutsAction>::new_stateless(move |_| {
            todo!();
        }));

        actions.add_action(RelmAction::<HelpAction>::new_stateless(move |_| {
            todo!();
        }));

        actions.add_action(RelmAction::<AboutAction>::new_stateless(move |_| {
            AboutDialog::builder().launch(()).detach();
        }));

        actions.add_action(RelmAction::<QuitAction>::new_stateless(move |_| {
            relm4::main_application().quit();
        }));

        actions
    };

    relm4::main_application().set_accelerators_for_action::<ShortcutsAction>(&["<Ctrl>question"]);
    relm4::main_application().set_accelerators_for_action::<HelpAction>(&["F1"]);
    relm4::main_application().set_accelerators_for_action::<QuitAction>(&["<Ctrl>q"]);

    actions.register_for_widget(widget);
}
