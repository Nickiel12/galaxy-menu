
#[derive(Debug)]
pub enum DBusMessage {
    DesktopNext,
    DesktopPrev,
    DesktopSet(u8),
}