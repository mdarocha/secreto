use relm4;
use relm4::actions::AccelsPlus;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::gtk::gio::Menu;
use relm4::gtk::{prelude::ApplicationExt, Application};

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

pub fn init_primary_menu_actions(app: &Application) -> RelmActionGroup<WindowActionGroup> {
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
            todo!();
        }));

        {
            let app = app.clone();
            actions.add_action(RelmAction::<QuitAction>::new_stateless(move |_| {
                app.quit();
            }));
        }

        actions
    };

    app.set_accelerators_for_action::<ShortcutsAction>(&["<Ctrl>question"]);
    app.set_accelerators_for_action::<HelpAction>(&["F1"]);
    app.set_accelerators_for_action::<QuitAction>(&["<Ctrl>q"]);

    actions
}
