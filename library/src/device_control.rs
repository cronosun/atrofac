use crate::err::AfErr;
use log::warn;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::CreateFileW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winnt::HANDLE;

pub struct DeviceControl {
    handle: HANDLE,
}

impl DeviceControl {
    pub fn new(file: &str) -> Result<Self, AfErr> {
        let wide_file: Vec<u16> = OsStr::new(file).encode_wide().chain(once(0)).collect();
        let file_handle = unsafe {
            CreateFileW(
                wide_file.as_ptr(),
                0xc0000000,
                3,
                null_mut(),
                3,
                0,
                null_mut(),
            )
        };
        if file_handle != null_mut() {
            Ok(Self {
                handle: file_handle,
            })
        } else {
            Err(format!("Unable to open device file '{}'.", file).into())
        }
    }

    pub fn control(
        &mut self,
        control_code: u32,
        in_buffer: &mut [u8],
        out_buffer: &mut [u8],
    ) -> Result<ControlResult, AfErr> {
        let in_buffer_size = u32::try_from(in_buffer.len())?;
        let out_buffer_size = u32::try_from(out_buffer.len())?;
        let mut out_buffer_written: u32 = 0;
        let out_buffer_written_ref: &mut u32 = &mut out_buffer_written;

        let in_buffer_c_void: *mut c_void = in_buffer as *mut _ as *mut c_void;
        let out_buffer_c_void: *mut c_void = out_buffer as *mut _ as *mut c_void;
        let success = unsafe {
            DeviceIoControl(
                self.handle,
                control_code,
                in_buffer_c_void,
                in_buffer_size,
                out_buffer_c_void,
                out_buffer_size,
                out_buffer_written_ref,
                null_mut(),
            )
        };
        if success != 0 {
            Ok(ControlResult { out_buffer_written })
        } else {
            let last_error = unsafe { GetLastError() };
            Err(format!(
                "Unable to write command {} to file (DeviceIoControl). \
            Last error code: {}, \
            In-Buffer {:x?}.",
                control_code, last_error, in_buffer
            )
            .into())
        }
    }

    fn dispose(&mut self) -> Result<(), AfErr> {
        let success = unsafe { CloseHandle(self.handle) };
        if success != 0 {
            Ok(())
        } else {
            Err("Unable to close file".into())
        }
    }
}

impl Drop for DeviceControl {
    fn drop(&mut self) {
        if let Err(err) = self.dispose() {
            warn!("Unable to close file: {:?}", err)
        }
    }
}

pub struct ControlResult {
    out_buffer_written: u32,
}

impl ControlResult {
    pub fn out_buffer_written(&self) -> u32 {
        self.out_buffer_written
    }
}
