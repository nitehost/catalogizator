use gtk::glib;
use gtk::prelude::*;
use gtk4 as gtk;

const APP_ID: &str = "ru.nitehost.Catalogizator";
const WIN_ID: &str = "CatalogizatorApp";
const UI_FILE: &str = "res/window.ui";
const CSS_FILE: &str = "res/style.css";
const DATABASE: &str = "db/collections.db";
const DATEFORMAT: &str = "%Y-%m-%d %H:%M:%S";
const NOUI: &str = "No Gtk-object in UI-file";

mod gui;
mod collection;
// mod entry;
// mod group;
// mod status;
// mod template;


fn main() -> glib::ExitCode {

    // приложение
    let app = gtk::Application::builder().application_id(APP_ID).build();

    // присоединение к сигналу activate
    app.connect_activate(gui::build_ui);

    // действия
    let actions = gui::get_app_actions();
    actions.iter().for_each(|action| {
        app.add_action(action);
    });

    // горячие клавиши
    let accels: Vec<(&str, &str)> = gui::get_app_accels();
    accels.iter().for_each(|accel| {
        app.set_accels_for_action(accel.0, &[accel.1]);
    });

    // запуск приложения
    app.run()

}
