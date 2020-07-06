mod implementation;

use crate::system::implementation::SystemImpl;
use atrofac_library::AfErr;
use std::borrow::Cow;
use std::path::PathBuf;
use std::time::Duration;

pub fn new_system_interface() -> Result<impl SystemInterface, AfErr> {
    SystemImpl::new()
}

pub trait SystemInterface {
    fn tray_clear(&mut self) -> Result<(), AfErr>;
    fn tray_add(&mut self, item: MenuItem) -> Result<(), AfErr>;
    fn tray_tooltip(&mut self, text: &str) -> Result<(), AfErr>;
    fn tray_icon(&mut self, buf: &[u8], width: u32, height: u32) -> Result<(), AfErr>;

    fn show_err_message(&mut self, title: &str, text: &str) -> Result<(), AfErr>;
    fn set_timer(&mut self, duration: Duration) -> Result<(), AfErr>;
    fn remove_timer(&mut self) -> Result<(), AfErr>;

    /// Opens the (system) editor to edit the given file.
    fn edit(&self, file: &PathBuf) -> Result<(), AfErr>;

    /// Gets next event (blocks until there's a new event). Returns None if there
    /// are no more events.
    fn receive_event(&self) -> Result<Option<SystemEvent>, AfErr>;

    /// quits the "system" -> will yield `None` in `receive_event`.
    fn quit(&self) -> Result<(), AfErr>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MenuItemIdx(u32);

impl MenuItemIdx {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

pub enum MenuItem<'a> {
    Separator,
    String(StringMenuItem<'a>),
}

pub struct StringMenuItem<'a> {
    pub text: Cow<'a, str>,
    pub state: MenuItemState,
}

pub enum SystemEvent {
    OnTimer,
    OnTray(MenuItemIdx),
    OnApmResume,
}

pub enum MenuItemState {
    Default,
    Checked,
}
