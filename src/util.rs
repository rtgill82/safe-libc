//
// Created:  Sat 18 Apr 2020 03:54:24 AM PDT
// Modified: Fri 21 Nov 2025 12:16:12 PM PST
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

use std::mem::MaybeUninit;
use crate::errno::Result;
use crate::posix::unistd;
use crate::stdlib::realloc;
use crate::VoidPtr;

#[repr(i32)]
pub(crate) enum BufType {
    Group  = libc::_SC_GETGR_R_SIZE_MAX,
    Passwd = libc::_SC_GETPW_R_SIZE_MAX
}

pub(crate) fn zeroed<T>() -> T {
    unsafe {
        MaybeUninit::zeroed().assume_init()
    }
}

pub(crate) fn get_bufsize(buftype: BufType) -> usize {
    if let Ok(rv) = unistd::sysconf(buftype as i32) {
        return rv as usize;
    }
    libc::BUFSIZ as usize
}

pub(crate) fn shrink_buf(ptr: VoidPtr) -> Result<VoidPtr> {
    unsafe {
        let len = libc::strlen(ptr as *const i8);
        realloc(ptr, len)
    }
}
