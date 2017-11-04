use std::ptr;
use std::os::raw::c_void;

use error::{ChainNull, Result};

mod fcntl;
mod libinput;
mod udev;
mod unistd;

unsafe extern "C" fn open_restricted(path: *const i8, flags: i32, _: *mut c_void) -> i32 {
    fcntl::open(path, flags)
}

unsafe extern "C" fn close_restricted(fd: i32, _: *mut c_void) {
    unistd::close(fd);
}

pub struct InputContext {
    interface: Box<libinput::libinput_interface>,
    libinput: *mut libinput::libinput,
    udev: *mut udev::udev,
}

impl InputContext {
    pub fn new() -> Result<Self> {
        let (udev, libinput, interface) = unsafe {
            let udev = udev::udev_new().chain_null(|| "Failed to create udev context")?;

            let mut interface = libinput::libinput_interface {
                open_restricted: Some(open_restricted),
                close_restricted: Some(close_restricted),
            };
            let mut interface = Box::new(interface);

            let inner = libinput::libinput_udev_create_context(
                interface.as_mut(),
                ptr::null_mut(),
                udev as *mut libinput::udev,
            ).chain_null(|| "Failed to create libinput context")?;

            (udev, inner, interface)
        };

        Ok(InputContext {
            udev,
            libinput,
            interface,
        })
    }
}

impl Drop for InputContext {
    fn drop(&mut self) {
        unsafe {
            udev::udev_unref(self.udev);
            libinput::libinput_unref(self.libinput);
        }
    }
}
