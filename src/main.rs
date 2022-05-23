
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

use zbus::Connection;

mod modules;

//https://gtk-rs.org/gtk4-rs/git/book/widgets.html

#[async_std::main]
async fn main() {

    let conn = Connection::session().await.unwrap();

    let proxy = modules::galaxymenuawesome::MyGreeter1Proxy::new(&conn).await.unwrap();
    let reply = proxy.say_hello("world").await.unwrap();

    dbg!(&reply);
    println!("{}", reply);

    let app = Application::builder()
    .application_id("org.galaxymenu.constellation")
    .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    // app.run();
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
