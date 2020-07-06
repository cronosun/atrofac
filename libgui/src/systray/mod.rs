use crate::systray::win32::Window;
use atrofac_library::AfErr;

#[allow(non_snake_case, unused)]
mod win32;

// Systray Lib

#[derive(Clone, Debug)]
pub enum SystrayError {
    OsError(String),
}

pub enum SystrayAction {
    SelectItem,
    DisplayMenu,
    HideMenu,
    Timer,
    Quit,
    ApmResume,
}

pub struct SystrayEvent {
    pub action: SystrayAction,
    pub menu_index: u32,
}

impl std::fmt::Display for SystrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &SystrayError::OsError(ref err_str) => write!(f, "OsError: {}", err_str),
        }
    }
}

impl Into<AfErr> for SystrayError {
    fn into(self) -> AfErr {
        AfErr::from(format!("Systray error: {}", self))
    }
}

pub struct Application {
    pub window: Window,
}

impl Application {
    pub fn new() -> Result<Application, SystrayError> {
        match Window::new() {
            Ok(w) => Ok(Application { window: w }),
            Err(e) => Err(e),
        }
    }
}

type Callback = Box<dyn Fn(&Window) -> () + 'static>;

fn make_callback<F>(f: F) -> Callback
where
    F: std::ops::Fn(&Window) -> () + 'static,
{
    Box::new(f) as Callback
}
