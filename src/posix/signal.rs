//
// Created:  Mon 22 Dec 2025 06:12:39 PM PST
// Modified: Tue 23 Dec 2025 02:45:40 PM PST
//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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
use libc::{pid_t,c_int};
use libc::sighandler_t;
use crate::errno::{Error,Result};

pub fn kill(pid: pid_t, sig: c_int) -> Result<()> {
    unsafe {
        if libc::kill(pid, sig) == 0 {
            Ok(())
        } else {
            Err(Error::errno())
        }
    }
}

pub fn sigaction(signum: c_int, act: &libc::sigaction)
    -> Result<libc::sigaction>
{
    unsafe {
        let mut oldact: libc::sigaction = MaybeUninit::zeroed().assume_init();
        let oldact_p: *mut libc::sigaction = &mut oldact;
        if libc::sigaction(signum, act, oldact_p) == 0 {
            Ok(oldact)
        } else {
            Err(Error::errno())
        }
    }
}

pub fn signal(signum: c_int, handler: sighandler_t)
    -> Result<sighandler_t>
{
    unsafe {
        let rv = libc::signal(signum, handler);
        if rv == libc::SIG_ERR {
            Err(Error::errno())
        } else {
            Ok(rv)
        }
    }
}
