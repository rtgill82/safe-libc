//
// Created:  Thu 16 Apr 2020 01:20:13 PM PDT
// Modified: Sun 19 Apr 2020 07:44:00 PM PDT
//
// Copyright (C) 2020 Robert Gill <locke@sdf.org>
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

use crate::errno;
use crate::errno::{Error,Result};
use crate::VoidPtr;

#[cfg(target_family = "unix")]
use crate::posix::string::strerror_s;

#[cfg(target_family = "windows")]
use crate::windows::string::strerror_s;

macro_rules! try_alloc {
    ($fn:expr) => {
        let ptr = $fn;
        match ptr.is_null() {
            false => return Ok(ptr),
            true  => {
                let errnum = errno::errno();
                match strerror_s(errnum) {
                    Ok(errmsg) => return Err(Error::new_msg(errnum, errmsg)),
                    Err(err) => return Err(err)
                }
            }
        }
    }
}

pub fn malloc(size: usize) -> Result<VoidPtr> {
    unsafe {
        try_alloc!(libc::malloc(size));
    }
}

pub fn realloc(ptr: VoidPtr, size: usize) -> Result<VoidPtr> {
    unsafe {
        try_alloc!(libc::realloc(ptr, size));
    }
}
