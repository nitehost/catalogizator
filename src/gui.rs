use gtk4 as gtk;
use gtk::prelude::*;
use gtk::gio;


pub fn get_window(win_id: &str, ui_file: &str) -> gtk::Window {

    // загружаем шаблон из UI-файла
    let builder = gtk::Builder::from_file(ui_file);

    // получаем из UI объект окна
    let window: gtk::Window = builder.object(win_id).expect(crate::NOUI);

    window
}


pub fn get_app_accels() -> Vec<(&'static str, &'static str)> {
    let mut accels = vec![];

    accels.push(("window.close", "<Ctrl>Q"));
    accels.push(("app.open_entry_form", "<Ctrl>N"));

    accels
}


pub fn get_app_actions() -> Vec<gio::SimpleAction> {

    // вектор всех действий приложения
    let mut actions = vec![];

    // открыть форму добавления записи
    let action = gio::SimpleAction::new("open_entry_form", None);
    action.connect_activate(|_,_| {
        let win = get_window("CatalogEntryWindow", "res/entry.ui");
        // win.set_application(Some(&app));
        win.present();
    });
    actions.push(action);

    actions
}


pub fn build_ui(app: &gtk::Application) {

    // загружаем окно из UI-файла
    let builder = gtk::Builder::from_file(crate::UI_FILE);

    // получаем из UI объект окна
    let window: gtk::ApplicationWindow = builder.object(crate::WIN_ID).expect(crate::NOUI);

    // создаём CSS-провайдер
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path(crate::CSS_FILE);

    // применяем стили
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("No display"),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    // заполнить список коллекций
    if let Ok(collections_model) = crate::collection::get_collections_model() {
        let collections_list: gtk::DropDown = builder.object("collections-list").expect(crate::NOUI);
        collections_list.set_model(Some(&collections_model));
    }

    // устанавливаем связь окна с приложением
    window.set_application(Some(app));

    // показать окно
    window.present();

}
