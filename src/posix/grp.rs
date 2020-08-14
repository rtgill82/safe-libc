//
// Created:  Thu 16 Apr 2020 01:57:09 PM PDT
// Modified: Fri 14 Aug 2020 01:26:17 PM PDT
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

use std::ffi::CStr;
use std::ptr;

use crate::errno::{Error,Result};
use crate::posix::unistd;
use crate::stdlib::realloc;
use crate::util::zeroed;

pub struct Group {
    grp: libc::group,
    buf: *mut libc::c_void
}

impl Group {
    pub fn gr_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.grp.gr_name) }
    }

    pub fn gr_passwd(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.grp.gr_passwd) }
    }

    pub fn gr_gid(&self) -> libc::gid_t {
        self.grp.gr_gid
    }

    pub fn gr_mem(&self) -> Option<Vec<&CStr>> {
        unsafe {
            if (*self.grp.gr_mem).is_null() {
                return None;
            }

            let mut vec = Vec::new();
            let mut p = self.grp.gr_mem;
            while !(*p).is_null() {
                vec.push(CStr::from_ptr(*p));
                p = p.add(1);
            }

            Some(vec)
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe { libc::free(self.buf); }
    }
}

pub fn getgid() -> libc::gid_t {
    unsafe { libc::getgid() }
}

pub fn getgrgid(gid: libc::gid_t) -> Result<Option<Group>> {
    unsafe {
        let mut grp: libc::group = zeroed();
        let mut result: *mut libc::group = ptr::null_mut();
        let mut bufsize = getgr_r_size_max();
        let mut buf = ptr::null_mut();

        loop {
            buf = realloc(buf, bufsize)?;
            let buf_i8 = buf as *mut i8;
            let rv = libc::getgrgid_r(gid, &mut grp, buf_i8, bufsize, &mut result);
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
        Ok(Some(Group { grp, buf }))
    }
}

fn getgr_r_size_max() -> usize {
    use libc::_SC_GETGR_R_SIZE_MAX;

    if let Ok(rv) = unistd::sysconf(_SC_GETGR_R_SIZE_MAX) {
        return rv as usize;
    }
    libc::BUFSIZ as usize
}
