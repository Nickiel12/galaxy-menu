
use zbus::{Connection, dbus_proxy};
use async_trait::async_trait;

pub struct DbusConnection<'a> {
    proxy: KWinProxy<'a>,
}

#[async_trait]
pub trait DbusProxy {
    async fn init<'a>(connection: &Connection) -> Result<DbusConnection<'a>, Box<dyn std::error::Error>>;
    async fn next_desktop(&self) -> ();
    async fn prev_desktop(&self) -> ();
}

//#[cfg(feature = "kde")]
#[async_trait]
impl DbusProxy for DbusConnection<'_> {
    async fn init<'a>(connection: &Connection) -> Result<DbusConnection<'a>, Box<dyn std::error::Error>> {
        let proxy = KWinProxy::new(connection).await?;

        return Ok(DbusConnection {
            proxy,
        })
    }
    
    async fn next_desktop(&self){
        match self.proxy.next_desktop().await {
            Ok(result) => {println!("{:?}", result)},
            Err(err) => {println!("{:?}", err)}
        }
    }

    async fn prev_desktop(&self){
        match self.proxy.previous_desktop().await {
            Ok(result) => {println!("{:?}", result)},
            Err(err) => {println!("{:?}", err)}
        }
    }

}

#[cfg(feature = "awesome")]
impl DbusProxy {
    pub async fn init() {

    }
    
    pub async fn next_desktop(){

    }

    pub async fn prev_desktop(){

    }
}

#[cfg(feature = "gnome")]
impl DbusProxy {
    pub async fn init() {

    }
    
    pub async fn next_desktop(){

    }

    pub async fn prev_desktop(){

    }
}

//#[cfg(feature = "kde")]
#[dbus_proxy(interface = "org.kde.KWin",
            default_service = "org.kde.KWin",
            default_path = "/KWin")]
trait KWin {
    /// cascadeDesktop method
    #[dbus_proxy(name="cascadeDesktop")]
    fn cascade_desktop(&self) -> zbus::Result<()>;

    /// currentDesktop method
    #[dbus_proxy(name="currentDesktop")]
    fn current_desktop(&self) -> zbus::Result<i32>;

    /// getWindowInfo method
    #[dbus_proxy(name="getWindowInfo")]
    fn get_window_info(
        &self,
        arg_1: &str,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// killWindow method
    #[dbus_proxy(name="killWindow")]
    fn kill_window(&self) -> zbus::Result<()>;

    /// nextDesktop method
    #[dbus_proxy(name="nextDesktop")]
    fn next_desktop(&self) -> zbus::Result<()>;

    /// previousDesktop method
    #[dbus_proxy(name="previousDesktop")]
    fn previous_desktop(&self) -> zbus::Result<()>;

    /// queryWindowInfo method
    #[dbus_proxy(name="queryWindowInfo")]
    fn query_window_info(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// reconfigure method
    #[dbus_proxy(name="reconfigure")]
    fn reconfigure(&self) -> zbus::Result<()>;

    /// replace method
    #[dbus_proxy(name="replace")]
    fn replace(&self) -> zbus::Result<()>;

    /// setCurrentDesktop method
    #[dbus_proxy(name="setCurrentDesktop")]
    fn set_current_desktop(&self, desktop: i32) -> zbus::Result<bool>;

    /// showDebugConsole method
    #[dbus_proxy(name="showDebugConsole")]
    fn show_debug_console(&self) -> zbus::Result<()>;

    /// startActivity method
    #[dbus_proxy(name="startActivity")]
    fn start_activity(&self, arg_1: &str) -> zbus::Result<bool>;

    /// stopActivity method
    #[dbus_proxy(name="stopActivity")]
    fn stop_activity(&self, arg_1: &str) -> zbus::Result<bool>;

    /// supportInformation method
    #[dbus_proxy(name="supportInformation")]
    fn support_information(&self) -> zbus::Result<String>;

    /// unclutterDesktop method
    #[dbus_proxy(name="unclutterDesktop")]
    fn unclutter_desktop(&self) -> zbus::Result<()>;

    /// reloadConfig signal
    #[dbus_proxy(signal)]
    fn reload_config(&self) -> zbus::Result<()>;
}


#[cfg(feature = "awesome")]
#[dbus_proxy(
    interface = "org.galaxymenu.MyGreeter",
    default_service = "org.galaxymenu.MyGreeter",
    default_path = "/org/galaxymenu/MyGreeter"
)]
trait MyGreeter {
    /// GoAway method
    fn go_away(&self) -> zbus::Result<()>;

    /// NextDesktop method
    fn next_desktop(&self) -> zbus::Result<()>;

    /// PrevDesktop method
    fn prev_desktop(&self) -> zbus::Result<()>;

    /// SayHello method
    fn say_hello(&self, name: &str) -> zbus::Result<String>;

    /// GreetedEveryone signal
    #[dbus_proxy(signal)]
    fn greeted_everyone(&self) -> zbus::Result<()>;

    /// GreeterName property
    #[dbus_proxy(property)]
    fn greeter_name(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_greeter_name(&self, value: &str) -> zbus::Result<()>;
}
