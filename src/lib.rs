#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate winit;

pub use event::{DeviceId, Event, EventType, ScanCode, WindowId};

mod error;
mod event;
#[cfg(unix)]
pub mod unix;
