//
// Created:  Sat 18 Apr 2020 02:58:32 AM PDT
// Modified: Sat 18 Apr 2020 02:27:19 PM PDT
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
type __errno_fn = unsafe extern "C" fn() -> *mut i32;

#[allow(non_upper_case_globals)]
#[cfg(target_os = "linux")]
const __errno: __errno_fn = libc::__errno_location;

#[allow(non_upper_case_globals)]
#[cfg(all(not(target_os = "linux")))]
const __errno: __errno_fn = libc::__errno;

pub fn errno() -> i32 {
    unsafe { *__errno() }
}

pub fn zero() {
    unsafe { *__errno() = 0 };
}

#[cfg(test)]
mod tests {
    use crate::errno;
    use super::__errno;

    #[test]
    fn test_errno() {
        unsafe { *__errno() = libc::EACCES; }
        assert_eq!(errno::errno(), libc::EACCES);
    }

    #[test]
    fn test_zero() {
        unsafe { *__errno() = libc::EACCES; }

        errno::zero();
        assert_eq!(errno::errno(), 0);
    }
}
