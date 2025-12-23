//
// Created:  Fri 17 Apr 2020 09:29:29 PM PDT
// Modified: Tue 23 Dec 2025 12:48:38 PM PST
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

use libc::{c_int,c_long};

use crate::errno::{Error,Result};
use crate::errno;

pub fn sysconf(name: c_int) -> Result<Option<c_long>> {
    unsafe {
        errno::zero();

        let rv = libc::sysconf(name);
        if rv == -1 && errno::errno() == 0 {
            Ok(None)
        } else if rv == -1 {
            Err(Error::errno())
        } else {
            Ok(Some(rv))
        }
    }
}
