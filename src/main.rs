use gtk::glib;
use gtk::prelude::*;

mod modules;
use modules::dbus_handler::DbusHandlerReturn;
use modules::Messages::DBusMessage;


fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("warp_gate.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(glade_src).unwrap();

    let window: gtk::ApplicationWindow = builder.object("window").unwrap();
    window.set_application(Some(application));


    let dbus = DbusHandlerReturn::start();
    let message_channel = dbus.send_channel.clone();

    let button: gtk::Button = builder.object("button").unwrap();

    button.connect_clicked(move |_| {
        message_channel.send(DBusMessage::DesktopNext).unwrap();
        println!("hello world");
    });

    window.stick();
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.ElnuDev.runic"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}