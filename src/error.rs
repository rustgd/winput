use std::ptr;

error_chain!{}

pub trait ChainNull: Sized {
    fn chain_null<F, EK>(self, callback: F) -> Result<Self>
    where
        F: FnOnce() -> EK,
        EK: Into<ErrorKind>;
}

impl<T> ChainNull for *mut T {
    fn chain_null<F, EK>(self, callback: F) -> Result<Self>
    where
        F: FnOnce() -> EK,
        EK: Into<ErrorKind>,
    {
        if self == ptr::null_mut() {
            Err(Error::from("Pointer was null")).chain_err(callback)
        } else {
            Ok(self)
        }
    }
}

impl<T> ChainNull for *const T {
    fn chain_null<F, EK>(self, callback: F) -> Result<Self>
        where
            F: FnOnce() -> EK,
            EK: Into<ErrorKind>,
    {
        if self == ptr::null() {
            Err(Error::from("Pointer was null")).chain_err(callback)
        } else {
            Ok(self)
        }
    }
}
