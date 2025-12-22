//
// Created:  Thu 16 Apr 2020 01:20:05 PM PDT
// Modified: Mon 22 Dec 2025 03:47:15 PM PST
//
// Copyright (C) 2020 Robert Gill <rtgill82@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies of the Software, its documentation and marketing & publicity
// materials, and acknowledgment shall be given in the documentation, materials
// and software packages that this Software was used.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

use std::ffi::{CStr,CString};
use std::ptr;

use crate::errno::{Error,Result};
use crate::stdlib::realloc;
use crate::util::*;
use crate::grp;

pub use libc::uid_t;

pub struct Passwd {
    pwd: libc::passwd,
    buf: *mut libc::c_void
}

impl Passwd {
    pub fn pw_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.pwd.pw_name) }
    }

    pub fn pw_passwd(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.pwd.pw_passwd) }
    }

    pub fn pw_uid(&self) -> uid_t {
        self.pwd.pw_uid
    }

    pub fn pw_gid(&self) -> grp::gid_t {
        self.pwd.pw_gid
    }

    pub fn pw_gecos(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.pwd.pw_gecos) }
    }

    pub fn pw_dir(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.pwd.pw_dir) }
    }

    pub fn pw_shell(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.pwd.pw_shell) }
    }
}

impl Drop for Passwd {
    fn drop(&mut self) {
        unsafe { libc::free(self.buf); }
    }
}

pub fn getuid() -> uid_t {
    unsafe { libc::getuid() }
}

pub fn geteuid() -> uid_t {
    unsafe { libc::geteuid() }
}

pub fn getpwnam<T: Into<Vec<u8>>>(name: T) -> Result<Option<Passwd>> {
    let cstring = CString::new(name)?;
    let cstr = cstring.as_c_str();

    unsafe {
        let mut pwd: libc::passwd = zeroed();
        let mut result: *mut libc::passwd = ptr::null_mut();
        let mut bufsize = get_bufsize(BufType::Passwd);
        let mut buf = ptr::null_mut();

        loop {
            buf = realloc(buf, bufsize)?;
            let buf_i8 = buf as *mut i8;
            let rv = libc::getpwnam_r(cstr.as_ptr(), &mut pwd,
                                      buf_i8, bufsize, &mut result);
            if !result.is_null() {
                break;
            }

            match rv {
                0 => return Ok(None),
                _ => {
                    if rv != libc::ERANGE {
                        return Err(Error::new(rv));
                    }
                    bufsize *= 2;
                }
            }
        }
        Ok(Some(Passwd { pwd, buf }))
    }
}

pub fn getpwuid(uid: uid_t) -> Result<Option<Passwd>> {
    unsafe {
        let mut pwd: libc::passwd = zeroed();
        let mut result: *mut libc::passwd = ptr::null_mut();
        let mut bufsize = get_bufsize(BufType::Passwd);
        let mut buf = ptr::null_mut();

        loop {
            buf = realloc(buf, bufsize)?;
            let buf_i8 = buf as *mut i8;
            let rv = libc::getpwuid_r(uid, &mut pwd, buf_i8, bufsize, &mut result);
            if !result.is_null() {
                break;
            }

            match rv {
                0 => return Ok(None),
                _ => {
                    if rv != libc::ERANGE {
                        return Err(Error::new(rv));
                    }
                    bufsize *= 2;
                }
            }
        }
        Ok(Some(Passwd { pwd, buf }))
    }
}

/*
pub fn setreuid() -> Result<> {
}
*/
