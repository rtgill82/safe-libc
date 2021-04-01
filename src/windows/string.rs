//
// Created:  Sat 18 Apr 2020 03:10:57 AM PDT
// Modified: Sun 19 Apr 2020 09:19:22 PM PDT
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

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::slice;

use crate::errno::Result;

extern "C" {
    fn _wcserror(errnum: libc::c_int) -> *const libc::wchar_t;
}

#[allow(non_upper_case_globals)]
pub const strerror_s: fn(i32) -> Result<String> = strerror;

pub fn strerror(errno: i32) -> Result<String> {
    unsafe {
        // _wcserror is thread safe
        let msg = _wcserror(errno);
        let len = libc::wcslen(msg);
        let msg = slice::from_raw_parts(msg, len);
        Ok(OsString::from_wide(msg).to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::strerror;

    #[test]
    fn test_strerror() {
        let msg = strerror(libc::EACCES);
        assert_eq!(msg, Ok(String::from("Permission denied")));
    }
}
