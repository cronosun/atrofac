use std::convert::TryFrom;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use msgbox::IconType;

use atrofac_library::AfErr;

use crate::system::{MenuItem, MenuItemIdx, MenuItemState, SystemEvent, SystemInterface};
use crate::systray::{Application, SystrayAction, SystrayError};

pub struct SystemImpl {
    app: Application,
}

impl SystemImpl {
    pub fn new() -> Result<Self, AfErr> {
        let app = map_err(Application::new())?;
        Ok(Self { app })
    }
}

impl SystemInterface for SystemImpl {
    fn tray_clear(&mut self) -> Result<(), AfErr> {
        map_err(self.app.window.clear_menu())?;
        Ok(())
    }

    fn tray_add(&mut self, item: MenuItem) -> Result<(), AfErr> {
        match item {
            MenuItem::Separator => {
                map_err(self.app.window.add_menu_separator())?;
            }
            MenuItem::String(string_menu_item) => {
                let index = map_err(self.app.window.add_menu_item(
                    &string_menu_item.text,
                    false,
                    |_| {},
                ))?;
                match string_menu_item.state {
                    MenuItemState::Default => {}
                    MenuItemState::Checked => {
                        map_err(self.app.window.select_menu_item(index))?;
                    }
                }
            }
        }
        Ok(())
    }

    fn tray_tooltip(&mut self, text: &str) -> Result<(), AfErr> {
        map_err(self.app.window.set_tooltip(text))
    }

    fn tray_icon(&mut self, buf: &[u8], width: u32, height: u32) -> Result<(), AfErr> {
        map_err(self.app.window.set_icon_from_buffer(buf, width, height))
    }

    fn show_err_message(&mut self, title: &str, text: &str) -> Result<(), AfErr> {
        msgbox::create(title, text, IconType::Error);
        Ok(())
    }

    fn set_timer(&mut self, duration: Duration) -> Result<(), AfErr> {
        map_err(
            self.app
                .window
                .set_timer(u32::try_from(duration.as_millis())?),
        )?;
        Ok(())
    }

    fn remove_timer(&mut self) -> Result<(), AfErr> {
        map_err(self.app.window.remove_timer())
    }

    fn edit(&self, file: &PathBuf) -> Result<(), AfErr> {
        Command::new("notepad.exe")
            .args(&[file.as_os_str()])
            .output()
            .map_err(|err| AfErr::from(format!("Unable to start editor: {}.", err)))?;
        Ok(())
    }

    fn receive_event(&self) -> Result<Option<SystemEvent>, AfErr> {
        loop {
            let systray_event =
                self.app.window.rx.recv().map_err(|err| {
                    AfErr::from(format!("Unable to get data from channel: {}.", err))
                })?;

            match systray_event.action {
                SystrayAction::SelectItem => {
                    return Ok(Some(SystemEvent::OnTray(MenuItemIdx::new(
                        systray_event.menu_index,
                    ))));
                }
                SystrayAction::DisplayMenu => {
                    // not interested in this
                }
                SystrayAction::HideMenu => {
                    // not interested in this
                }
                SystrayAction::Timer => {
                    return Ok(Some(SystemEvent::OnTimer));
                }
                SystrayAction::Quit => {
                    return Ok(None);
                }
                SystrayAction::ApmResume => return Ok(Some(SystemEvent::OnApmResume)),
            };
        }
    }

    fn quit(&self) -> Result<(), AfErr> {
        self.app.window.quit();
        Ok(())
    }
}

fn map_err<T>(result: Result<T, SystrayError>) -> Result<T, AfErr> {
    result.map_err(|err| {
        let af_err: AfErr = err.into();
        af_err
    })
}
