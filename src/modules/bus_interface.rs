
use zbus::Connection;

use crossbeam_channel::{Receiver, Sender};
use workctl::sync_flag;


use super::galaxymenuawesome::MyGreeterProxy;
use super::Messages::DBusMessage;

// I need a thread that can listen to dbus signals
// and also call dbus methods

// it needs an input channel and an output channel
pub struct DbusInterface {

    stop_thread: sync_flag::SyncFlagRx,
    incoming_signals: Receiver<DBusMessage>,
    gui_update_channel: Sender<DBusMessage>,

}

impl DbusInterface {
    pub fn start(stop_flag: sync_flag::SyncFlagRx, to_dbus_channel: Receiver<DBusMessage>, to_gui_channel: Sender<DBusMessage>) -> Self {
        let interface: DbusInterface = 
        DbusInterface {
            stop_thread: stop_flag,
            incoming_signals: to_dbus_channel,
            gui_update_channel: to_gui_channel,
        };

        return interface;
    }

    async fn main_thread() {
        let conn = Connection::session().await.unwrap();

        let proxy = MyGreeterProxy::new(&conn).await.unwrap();
        let reply = proxy.say_hello("ding").await.unwrap();
    }

}
