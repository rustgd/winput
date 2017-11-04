extern crate input;
extern crate libc;
extern crate winit;

pub use event::{DeviceId, Event, EventType, ScanCode, WindowId};

mod event;
pub mod unix;
