//! QuRT-specific extensions to primitives in the [`std::fs`] module.
//!
//! [`std::fs`]: crate::fs

#![stable(feature = "raw_ext", since = "1.1.0")]

use crate::fs::Metadata;

/// OS-specific extensions to [`fs::Metadata`].
///
/// [`fs::Metadata`]: crate::fs::Metadata
#[stable(feature = "raw_ext", since = "1.1.0")]
pub trait MetadataExt {
    /// Gain a reference to the underlying `stat` structure which contains
    /// the raw information returned by the OS.
    #[stable(feature = "raw_ext", since = "1.1.0")]
    fn as_raw_stat(&self) -> &raw::stat;
}

#[stable(feature = "raw_ext", since = "1.1.0")]
impl MetadataExt for Metadata {
    fn as_raw_stat(&self) -> &raw::stat {
        // QuRT doesn't have real file metadata
        static DUMMY_STAT: raw::stat = raw::stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atime: 0,
            st_atime_nsec: 0,
            st_mtime: 0,
            st_mtime_nsec: 0,
            st_ctime: 0,
            st_ctime_nsec: 0,
        };
        &DUMMY_STAT
    }
}

use super::raw;
