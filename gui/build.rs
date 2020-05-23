#[cfg(target_os = "windows")]
extern crate winres;

// only build for windows
#[cfg(target_os = "windows")]
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icon.ico");
        res.compile().unwrap();
    }
}

// nothing to do for other operating systems
#[cfg(not(target_os = "windows"))]
fn main() {}
