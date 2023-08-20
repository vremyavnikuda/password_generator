use gtk::prelude::*;
use crate::generate_password::generate_password;

pub(crate) fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    //(ширина x высота)
    window.set_default_size(400, 300);

    //вертикальный контейнер для элементов
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    
    let length_spin = gtk::SpinButton::with_range(1.0, 100.0, 1.0);
    length_spin.set_value(12.0);

    //флажки для выбора параметров генерации пароля
    let uppercase_checkbox = gtk::CheckButton::with_label("Большие буквы");
    let lowercase_checkbox = gtk::CheckButton::with_label("Маленькие буквы");
    let symbols_checkbox = gtk::CheckButton::with_label("Символы");
    let numbers_checkbox = gtk::CheckButton::with_label("Цифры");

    // Минимальная, максимальная и шаг длины пароля

    //Поле вывода для отображения сгенерированного пароля
    let password_entry = gtk::Entry::new();

    // Запретит редактирование поля вывода
    password_entry.set_editable(false);

    //"Generator"
    let generate_button = gtk::Button::with_label("Generator");

    // Клонируем поле вывода для передачи в замыкание
    let password_entry_clone = password_entry.clone();

    // Клонируем флажок для передачи в замыкание
    let uppercase_checkbox_clone = uppercase_checkbox.clone();
    let lowercase_checkbox_clone = lowercase_checkbox.clone();
    let symbols_checkbox_clone = symbols_checkbox.clone();
    let numbers_checkbox_clone = numbers_checkbox.clone();
    let length_spin_clone = length_spin.clone();

    generate_button.connect_clicked(move |_| {
        // Логика генерации пароля на основе выбранных параметров
        let use_uppercase = uppercase_checkbox_clone.is_active();
        let use_lowercase = lowercase_checkbox_clone.is_active();
        let use_symbols = symbols_checkbox_clone.is_active();
        let use_numbers = numbers_checkbox_clone.is_active();
        let password_length = length_spin_clone.value_as_int();

        // Другие параметры генерации пароля...
        let generated_password = generate_password(use_uppercase, use_lowercase, use_symbols, use_numbers, password_length);

        //сгенерированный пароль в поле вывода
        password_entry_clone.set_text(&generated_password);
    });

    vbox.append(&uppercase_checkbox);
    vbox.append(&lowercase_checkbox);
    vbox.append(&symbols_checkbox);
    vbox.append(&numbers_checkbox);
    vbox.append(&length_spin);
    vbox.append(&generate_button);
    vbox.append(&password_entry);

    //`vbox` как дочерний элемент окна
    window.set_child(Some(&vbox));

    window.present();
}


