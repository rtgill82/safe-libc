//
// Created:  Sat 18 Apr 2020 03:04:00 AM PDT
// Modified: Wed 26 Nov 2025 03:34:59 PM PST
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

use libc::c_int;

#[allow(non_camel_case_types)]
pub type errno_t = c_int;

extern "C" {
    fn _get_doserrno(errnum: *mut c_int) -> errno_t;
    fn _set_doserrno(errnum: c_int) -> errno_t;
    fn _get_errno(errnum: *mut c_int) -> errno_t;
    fn _set_errno(errnum: c_int) -> errno_t;
}

pub fn doserrno() -> c_int {
    unsafe {
        let mut errnum: c_int = 0;
        _get_doserrno(&mut errnum);
        errnum
    }
}

pub fn errno() -> c_int {
    unsafe {
        let mut errnum: c_int = 0;
        _get_errno(&mut errnum);
        errnum
    }
}

pub fn zero() {
    unsafe {
        _set_doserrno(0);
        _set_errno(0);
    }
}

#[cfg(test)]
mod tests {
    use crate::errno;
    use super::_set_doserrno;
    use super::_set_errno;

    #[test]
    fn test_doserrno() {
        unsafe { _set_doserrno(libc::EACCES); }
        assert_eq!(errno::doserrno(), libc::EACCES);
    }

    #[test]
    fn test_errno() {
        unsafe { _set_errno(libc::EACCES); }
        assert_eq!(errno::errno(), libc::EACCES);
    }

    #[test]
    fn test_zero() {
        unsafe {
            _set_doserrno(libc::EACCES);
            _set_errno(libc::EACCES);
        }

        errno::zero();
        assert_eq!(errno::doserrno(), 0);
        assert_eq!(errno::errno(), 0);
    }
}
