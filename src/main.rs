mod generate_password;
mod activate;

use gtk::prelude::*;
use crate::activate::on_activate;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let app_password_generator = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app_password_generator.connect_activate(on_activate);
    // Run the application
    app_password_generator.run();
}