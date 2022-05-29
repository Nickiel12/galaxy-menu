
use zbus::Connection;

use crossbeam_channel::{Sender, Receiver, bounded};
use async_std::task::block_on;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use workctl::sync_flag;

use super::dbus_proxy::{DbusProxy, DbusConnection};
use super::Messages::DBusMessage;

// I need a thread that can listen to dbus signals
// and also call dbus methods

// it needs an input channel and an output channel
pub struct DbusHandlerReturn {

    pub stop_signal: sync_flag::SyncFlagTx,
    pub send_channel: crossbeam_channel::Sender<DBusMessage>,
    pub receive_channel: crossbeam_channel::Receiver<DBusMessage>,
    pub thread_handle: thread::JoinHandle<()>,

}

impl DbusHandlerReturn {
    pub fn start() -> Self {
        
        let (stopflag_tx, stopflag_rx) = sync_flag::new_syncflag(false);
        let (to_dbus_thread_tx, to_dbus_thread_rx) = bounded::<DBusMessage>(20);
        let (from_dbus_thread_tx, from_dbus_thread_rx) = bounded::<DBusMessage>(20);

        let thread = main_thread(stopflag_rx, to_dbus_thread_rx, from_dbus_thread_tx);
        
        let interface = DbusHandlerReturn {
            stop_signal: stopflag_tx,
            send_channel: to_dbus_thread_tx,
            receive_channel: from_dbus_thread_rx,
            thread_handle: thread,
        };

        return interface;
    }

}

fn main_thread(stop_flag: sync_flag::SyncFlagRx, mut incoming: Receiver<DBusMessage>, mut outgoing: Sender<DBusMessage>) -> JoinHandle<()> {
    let conn = block_on(Connection::session()).unwrap();

    let proxy = block_on(DbusConnection::init(&conn)).unwrap();

    let handler = thread::spawn(move || {
        while !stop_flag.get() {
            block_on(poll_incoming(&mut incoming, &proxy));
        }
    });
    return handler;

    //let proxy = MyGreeterProxy::new(&conn).await.unwrap();
    //let reply = proxy.say_hello("ding").await.unwrap();
}

async fn poll_incoming<'a>(incoming: &mut Receiver<DBusMessage>, proxy: &DbusConnection<'a>){
    if !incoming.is_empty(){
        match incoming.recv_timeout(Duration::from_millis(100)) {
            Ok(message) => {
                let future = match message {
                    DBusMessage::DesktopNext => proxy.next_desktop(),
                    DBusMessage::DesktopPrev => proxy.prev_desktop(),
                    DBusMessage::DesktopSet(_) => todo!(),
                };
                block_on(future);
            },
            Err(_) => panic!("Poll incoming got a timeout error"),
        }
    }
}

async fn check_signals<'a>(outgoing: &mut Sender<DBusMessage>, proxy: &DbusConnection<'a>){
    //do something
}