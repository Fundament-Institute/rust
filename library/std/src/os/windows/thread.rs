//! Windows-specific extensions to primitives in the [`std::thread`] module.
//!
//! [`std::thread`]: crate::thread

#![stable(feature = "thread_extensions", since = "1.9.0")]

use core::marker::Leak;

use crate::os::windows::io::{AsRawHandle, IntoRawHandle, RawHandle};
use crate::sys_common::{AsInner, IntoInner};
use crate::thread;

#[stable(feature = "thread_extensions", since = "1.9.0")]
impl<T: Leak> AsRawHandle for thread::JoinHandle<T> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.as_inner().handle().as_raw_handle() as *mut _
    }
}

#[stable(feature = "thread_extensions", since = "1.9.0")]
impl<T: Leak> IntoRawHandle for thread::JoinHandle<T> {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.into_inner().into_handle().into_raw_handle() as *mut _
    }
}
