//
// Created:  Fri 17 Apr 2020 11:55:31 PM PDT
// Modified: Sat 18 Apr 2020 02:54:43 PM PDT
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

use std::ffi::CString;
use std::ptr;

use crate::stdlib::realloc;
use crate::errno::{Error,Result};
use crate::errno;

pub fn strerror(errno: i32) -> Result<String> {
    unsafe {
        let mut buflen: usize = libc::BUFSIZ as usize;
        let buf = ptr::null_mut();

        loop {
            let buf = realloc(buf, buflen)?;
            let rv = libc::strerror_r(errno, buf as *mut i8, buflen);
            match rv {
                0  => { // success
                    let buf = buf as *mut i8;
                    let string = CString::from_raw(buf).into_string().unwrap();
                    return Ok(string);
                },

                _  => { // failure
                    let errnum;
                    if rv == -1 {
                        errnum = errno::errno();
                    } else {
                        errnum = rv;
                    };

                    if errnum != libc::ERANGE {
                        libc::free(buf);
                        return Err(Error::new(errnum));
                    }

                    // else reallocate and try again
                    buflen *= 2;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::strerror;
    use crate::errno::Error;

    #[test]
    fn test_strerror() {
        let msg = strerror(libc::EACCES);
        assert_eq!(msg, Ok(String::from("Permission denied")));
    }

    #[test]
    fn test_strerror_invalid() {
        let msg = strerror(1234567890);
        assert_eq!(msg, Err(Error::new(libc::EINVAL)));
    }
}
