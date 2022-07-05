//! Helper functions for the [`cap-std` crate].
//!
//! [`cap-std` crate]: https://crates.io/crates/cap-std
//  SPDX-License-Identifier: Apache-2.0 OR MIT

use cap_std::fs::DirBuilder;
use cap_std::io_lifetimes::BorrowedFd;
use cap_std_ext::cap_std;
use cap_std_ext::cap_std::fs::Dir;
use std::ffi::OsStr;
use std::io::Result;
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::path::Path;

pub(crate) fn dirbuilder_from_mode(m: u32) -> DirBuilder {
    let mut r = DirBuilder::new();
    r.mode(m);
    r
}

#[allow(dead_code)]
/// Create a new cap-std Dir instance from an openat Dir instance.
pub(crate) fn from_openat(o: &openat::Dir) -> std::io::Result<Dir> {
    let o = unsafe { BorrowedFd::borrow_raw(o.as_raw_fd()) };
    Dir::reopen_dir(&o)
}

#[allow(dead_code)]
/// Create a new openat Dir instance from a cap-std Dir instance.
pub(crate) fn to_openat(o: &Dir) -> std::io::Result<openat::Dir> {
    let src = unsafe { openat::Dir::from_raw_fd(o.as_raw_fd()) };
    let r = src.sub_dir(".");
    let _ = src.into_raw_fd();
    r
}

/// Given a (possibly absolute) filename, return its parent directory and filename.
pub(crate) fn open_dir_of(
    path: &Path,
    ambient_authority: cap_std::AmbientAuthority,
) -> Result<(Dir, &OsStr)> {
    let parent = path
        .parent()
        .filter(|v| !v.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent = Dir::open_ambient_dir(parent, ambient_authority)?;
    let filename = path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "the source path does not name a file",
        )
    })?;
    Ok((parent, filename))
}
