//! QuRT-specific I/O functionality.

#![stable(feature = "raw_ext", since = "1.1.0")]

use crate::marker::PhantomData;
use crate::os::raw;

/// Owned file descriptor (placeholder for QuRT).
#[repr(transparent)]
#[stable(feature = "io_safety", since = "1.63.0")]
pub struct OwnedFd {
    fd: raw::c_int,
    _marker: PhantomData<*const ()>,
}

#[stable(feature = "io_safety", since = "1.63.0")]
impl Drop for OwnedFd {
    fn drop(&mut self) {
        // QuRT doesn't have real file descriptors
    }
}

/// Borrowed file descriptor (placeholder for QuRT).
#[repr(transparent)]
#[derive(Copy, Clone)]
#[stable(feature = "io_safety", since = "1.63.0")]
pub struct BorrowedFd<'fd> {
    fd: raw::c_int,
    _marker: PhantomData<&'fd ()>,
}
