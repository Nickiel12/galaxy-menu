
use zbus::dbus_proxy;

pub struct DbusProxy {
    
}

//#[cfg(feature = "kde")]
impl DbusProxy {
    pub fn init() {
        
    }
    
    pub fn next_desktop(){

    }

    pub fn prev_desktop(){

    }

}

#[cfg(feature = "awesome")]
impl DbusProxy {
    pub fn init() {

    }
    
    pub fn next_desktop(){

    }

    pub fn prev_desktop(){

    }
}

#[cfg(feature = "gnome")]
impl DbusProxy {
    pub fn init() {

    }
    
    pub fn next_desktop(){

    }

    pub fn prev_desktop(){

    }
}

//#[cfg(feature = "kde")]
#[dbus_proxy(interface = "org.kde.KWin",
            default_path = "/KWin")]
trait KWin {
    /// cascadeDesktop method
    fn cascade_desktop(&self) -> zbus::Result<()>;

    /// currentDesktop method
    fn current_desktop(&self) -> zbus::Result<i32>;

    /// getWindowInfo method
    fn get_window_info(
        &self,
        arg_1: &str,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// killWindow method
    fn kill_window(&self) -> zbus::Result<()>;

    /// nextDesktop method
    fn next_desktop(&self) -> zbus::Result<()>;

    /// previousDesktop method
    fn previous_desktop(&self) -> zbus::Result<()>;

    /// queryWindowInfo method
    fn query_window_info(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// reconfigure method
    fn reconfigure(&self) -> zbus::Result<()>;

    /// replace method
    fn replace(&self) -> zbus::Result<()>;

    /// setCurrentDesktop method
    fn set_current_desktop(&self, desktop: i32) -> zbus::Result<bool>;

    /// showDebugConsole method
    fn show_debug_console(&self) -> zbus::Result<()>;

    /// startActivity method
    fn start_activity(&self, arg_1: &str) -> zbus::Result<bool>;

    /// stopActivity method
    fn stop_activity(&self, arg_1: &str) -> zbus::Result<bool>;

    /// supportInformation method
    fn support_information(&self) -> zbus::Result<String>;

    /// unclutterDesktop method
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
