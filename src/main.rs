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

mod collection;
mod entry;
mod group;
mod status;
mod template;

fn main() -> glib::ExitCode {
    // приложение
    let app = gtk::Application::builder().application_id(APP_ID).build();

    // присоединение к сигналу activate
    app.connect_activate(on_activate);

    // запуск приложения
    app.run()
}

fn on_activate(app: &gtk::Application) {
    // загружаем окно из UI-файла
    let builder = gtk::Builder::from_file(UI_FILE);

    // получаем из UI объект окна
    let window: gtk::ApplicationWindow = builder.object(WIN_ID).expect(NOUI);

    // создаём CSS-провайдер
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path(CSS_FILE);

    // применяем стили
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("No display"),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    // заполнить список коллекций
    if let Ok(collections_model) = collection::get_collections_model() {
        let collections_list: gtk::DropDown = builder.object("collections-list").expect(NOUI);
        collections_list.set_model(Some(&collections_model));
    }

    // устанавливаем связь окна с приложением
    window.set_application(Some(app));

    // показать окно
    window.present();
}
