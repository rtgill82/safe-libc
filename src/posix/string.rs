//
// Created:  Fri 17 Apr 2020 11:55:31 PM PDT
// Modified: Tue 25 Nov 2025 03:44:25 PM PST
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

use std::ffi::CString;
use std::{ptr,slice};
use libc::c_void;

use crate::stdlib::realloc;
use crate::errno::{Error,Result};
use crate::errno;

const BUF_SIZE: usize = 512;

macro_rules! rv2errnum {
    ($rv:ident) => {
        match $rv {
             0 => 0,
            -1 => errno::errno(),
            _  => $rv
        }
    }
}

pub fn strerror(errnum: i32) -> Result<String> {
    unsafe {
        let mut buflen = BUF_SIZE;
        let buf = ptr::null_mut();

        loop {
            let buf = realloc(buf, buflen)?;
            let rv = libc::strerror_r(errnum, buf as *mut i8, buflen);
            let errnum = rv2errnum!(rv);

            if errnum == 0 { // success
                let string = c_void2string(buf)?;
                return Ok(string);
            }

            if errnum == libc::EINVAL {
                let string = c_void2string(buf)?;
                return Err(Error::new_msg(errnum, string));
            }

            // reallocate and try again
            buflen *= 2;
        }
    }
}

// strerror with static buffer
pub(crate) fn strerror_s(errnum: i32) -> Result<String> {
    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

    let rv = unsafe {
        libc::strerror_r(errnum, buf.as_mut_ptr() as *mut i8, BUF_SIZE)
    };

    let errnum = rv2errnum!(rv);
    if errnum == libc::ERANGE {
        buf[BUF_SIZE - 1] = 0;
    }

    let string = String::from_utf8_lossy(&buf).to_string();
    if errnum == libc::EINVAL {
        return Err(Error::new_msg(errnum, string));
    }
    Ok(string)
}

unsafe fn c_void2string(buf: *mut c_void) -> Result<String> {
    let len = libc::strlen(buf as *mut i8) + 1;
    let buf = buf as *const u8;
    let vec: Vec<u8> = slice::from_raw_parts(buf, len).into();
    let string = CString::from_vec_with_nul(vec)?
        .to_string_lossy().to_string();

    libc::free(buf as *mut c_void);
    Ok(string)
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
