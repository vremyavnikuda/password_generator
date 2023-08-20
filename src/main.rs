mod activate;
mod generate_password;

use gtk::prelude::*;
use crate::activate::on_activate;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let app = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();

    app.connect_activate(on_activate);

    // Run the application
    app.run();
}