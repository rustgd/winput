
use input;
use libc::c_void;

mod udev;

pub struct InputContext {
    inner: input::Libinput,
}

impl InputContext {
    pub fn new() -> Self {
        let inner = unsafe {
            let udev = udev::udev_new();

            let interface = input::LibinputInterface {
                open_restricted: None,
                close_restricted: None,
            };

            input::Libinput::new_from_udev::<()>(interface, None, udev as *mut c_void)
        };

        InputContext { inner }
    }
}
