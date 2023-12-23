//! # DBus interface proxy for: `org.kde.StatusNotifierItem`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `dbus-status-notifier-item.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.

// suppress warning from generated code
#![allow(clippy::type_complexity)]

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.kde.StatusNotifierItem", assume_defaults = true)]
trait StatusNotifierItem {
    /// Activate method
    fn activate(&self, x: i32, y: i32) -> zbus::Result<()>;

    /// ContextMenu method
    fn context_menu(&self, x: i32, y: i32) -> zbus::Result<()>;

    /// Scroll method
    fn scroll(&self, delta: i32, orientation: &str) -> zbus::Result<()>;

    /// SecondaryActivate method
    fn secondary_activate(&self, x: i32, y: i32) -> zbus::Result<()>;

    /// NewAttentionIcon signal
    #[dbus_proxy(signal)]
    fn new_attention_icon(&self) -> zbus::Result<()>;

    /// NewIcon signal
    #[dbus_proxy(signal)]
    fn new_icon(&self) -> zbus::Result<()>;

    /// NewOverlayIcon signal
    #[dbus_proxy(signal)]
    fn new_overlay_icon(&self) -> zbus::Result<()>;

    /// NewStatus signal
    #[dbus_proxy(signal)]
    fn new_status(&self, status: &str) -> zbus::Result<()>;

    /// NewTitle signal
    #[dbus_proxy(signal)]
    fn new_title(&self) -> zbus::Result<()>;

    /// NewToolTip signal
    #[dbus_proxy(signal)]
    fn new_tool_tip(&self) -> zbus::Result<()>;

    /// AttentionIconName property
    #[dbus_proxy(property)]
    fn attention_icon_name(&self) -> zbus::Result<String>;

    /// AttentionIconPixmap property
    #[dbus_proxy(property)]
    fn attention_icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// AttentionMovieName property
    #[dbus_proxy(property)]
    fn attention_movie_name(&self) -> zbus::Result<String>;

    /// Category property
    #[dbus_proxy(property)]
    fn category(&self) -> zbus::Result<String>;

    /// IconName property
    #[dbus_proxy(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    /// IconPixmap property
    #[dbus_proxy(property)]
    fn icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// IconThemePath property
    #[dbus_proxy(property)]
    fn icon_theme_path(&self) -> zbus::Result<String>;

    /// Id property
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    /// ItemIsMenu property
    #[dbus_proxy(property)]
    fn item_is_menu(&self) -> zbus::Result<bool>;

    /// Menu property
    #[dbus_proxy(property)]
    fn menu(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// OverlayIconName property
    #[dbus_proxy(property)]
    fn overlay_icon_name(&self) -> zbus::Result<String>;

    /// OverlayIconPixmap property
    #[dbus_proxy(property)]
    fn overlay_icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// Status property
    #[dbus_proxy(property)]
    fn status(&self) -> zbus::Result<String>;

    /// Title property
    #[dbus_proxy(property)]
    fn title(&self) -> zbus::Result<String>;

    /// ToolTip property
    #[dbus_proxy(property)]
    fn tool_tip(&self) -> zbus::Result<(String, Vec<(i32, i32, Vec<u8>)>)>;
}
