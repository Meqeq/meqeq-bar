use zbus::interface;

pub struct StatusNotifierHost {}

#[interface(name = "org.kde.StatusNotifierHost")]
impl StatusNotifierHost {}
