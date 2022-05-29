use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};


mod modules;
use modules::dbus_handler::DbusHandlerReturn;
use modules::Messages::DBusMessage;

//https://gtk-rs.org/gtk4-rs/git/book/widgets.html



#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let app = Application::builder()
    .application_id("org.galaxymenu.constellation")
    .build();


    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
    Ok(())
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


    let dbus = DbusHandlerReturn::start();
    let message_channel = dbus.send_channel.clone();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        message_channel.send(DBusMessage::DesktopNext).unwrap();
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
