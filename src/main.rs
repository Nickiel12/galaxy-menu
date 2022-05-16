use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

//https://gtk-rs.org/gtk4-rs/git/book/widgets.html
//https://develop.kde.org/docs/use/d-bus/introduction_to_dbus/
//https://www.reddit.com/r/kde/comments/8h92z0/command_line_command_to_switch_virtual_desktop/
//https://forum.kde.org/viewtopic.php?t=107875
//https://docs.rs/dbus/latest/dbus/

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.galaxymenu.constellation")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello Silence!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.set_decorated(false);
    // Present window
    window.present();
}
