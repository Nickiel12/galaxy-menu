

pub enum DBusMessage {
    DESKTOP_NEXT,
    DESKTOP_PREV,
    DESKTOP_SET(u8),
}