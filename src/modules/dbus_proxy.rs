
use super::proxys::kde;

pub struct DbusProxy {
    
}

#[cfg(feature = "kde")]
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
