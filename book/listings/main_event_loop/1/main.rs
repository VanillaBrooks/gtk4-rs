use std::time::Duration;

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Create a button
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal
    button.connect_clicked(move |_| {
        // GUI is blocked for 5 seconds after the button is pressed
        let five_seconds = Duration::from_secs(5);
        std::thread::sleep(five_seconds);
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
