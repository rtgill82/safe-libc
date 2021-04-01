//
// Created:  Sat 18 Apr 2020 03:04:00 AM PDT
// Modified: Sat 18 Apr 2020 01:38:15 PM PDT
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

#[allow(non_camel_case_types)]
pub type errno_t = libc::c_int;

extern "C" {
    fn __doserrno() -> *mut libc::c_ulong;
    fn _errno() -> *mut libc::c_int;
}

pub fn doserrno() -> u64 {
    unsafe { *__doserrno() as u64 }
}

pub fn errno() -> i32 {
    unsafe { *_errno() }
}

pub fn zero() {
    unsafe {
        *__doserrno() = 0;
        *_errno() = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::errno;
    use super::__doserrno;
    use super::_errno;

    #[test]
    fn test_doserrno() {
        unsafe { *__doserrno() = libc::EACCES as u32; }
        assert_eq!(errno::doserrno(), libc::EACCES as u64);
    }

    #[test]
    fn test_errno() {
        unsafe { *_errno() = libc::EACCES; }
        assert_eq!(errno::errno(), libc::EACCES);
    }

    #[test]
    fn test_zero() {
        unsafe {
            *__doserrno() = libc::EACCES as u32;
            *_errno() = libc::EACCES;
        }

        errno::zero();
        assert_eq!(errno::doserrno(), 0);
        assert_eq!(errno::errno(), 0);
    }
}
