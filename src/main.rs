use gtk4 as gtk;
use gtk::prelude::*;

const APP_ID: &str = "ru.nitehost.Catalogizator";
const WIN_ID: &str = "CatalogizatorApp";
const UI_FILE: &str = "res/window.ui";

fn main() -> glib::ExitCode {

    // приложение
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    // присоединение к сигналу activate
    app.connect_activate(on_activate);

    // запуск приложения
    app.run()
}

fn on_activate(app: &gtk::Application) {

    // загружаем окно из UI-файла
    let builder = gtk::Builder::from_file(UI_FILE);

    // получаем из UI объект окна
    let window: gtk::ApplicationWindow = builder.object(WIN_ID)
        .expect("No GtkApplicationWindow object in UI-file");

    // устанавливаем связь окна с приложением
    window.set_application(Some(app));

    // показать окно
    window.present();
}