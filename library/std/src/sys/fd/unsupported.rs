use crate::io;

pub struct FileDesc;

impl FileDesc {
    pub fn new(_raw: i32) -> Self {
        Self
    }

    pub fn into_raw(self) -> i32 {
        panic!("file descriptors not supported on this platform")
    }

    pub fn as_raw(&self) -> i32 {
        panic!("file descriptors not supported on this platform")
    }
}

pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

impl IsMinusOne for i32 {
    fn is_minus_one(&self) -> bool {
        *self == -1
    }
}

pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
    if t.is_minus_one() {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    } else {
        Ok(t)
    }
}

pub fn cvt_r<T, F>(mut f: F) -> io::Result<T>
where
    T: IsMinusOne,
    F: FnMut() -> T,
{
    loop {
        match cvt(f()) {
            Err(ref e) if e.is_interrupted() => {}
            other => return other,
        }
    }
}
