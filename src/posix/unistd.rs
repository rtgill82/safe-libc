//
// Created:  Fri 17 Apr 2020 09:29:29 PM PDT
// Modified: Sat 18 Apr 2020 05:01:06 PM PDT
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

use crate::errno::{Error,Result};

pub fn sysconf(name: i32) -> Result<i64> {
    unsafe {
        let rv = libc::sysconf(name);
        if rv == -1 {
            return Err(Error::errno());
        }
        return Ok(rv as i64);
    }
}
