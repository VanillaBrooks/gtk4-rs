mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

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
    let button = CustomButton::with_label("Press me!");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Connect to "clicked" signal
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
