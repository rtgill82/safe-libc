//
// Created:  Thu 16 Apr 2020 01:19:12 PM PDT
// Modified: Sun 19 Apr 2020 07:10:59 PM PDT
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

use std::{cmp,fmt};
use std::{error,result};

use crate::string::strerror;

#[cfg(target_family = "unix")]
#[doc(inline)]
pub use crate::posix::errno::*;

#[cfg(target_family = "windows")]
#[doc(inline)]
pub use crate::windows::errno::*;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    errmsg: String,
    errnum: i32
}

impl Error {
    pub fn new(errnum: i32) -> Error {
        match strerror(errnum) {
            Ok(errmsg) => Error { errmsg, errnum },
            Err(err) => err
        }
    }

    pub fn new_msg(errnum: i32, errmsg: String) -> Error {
        Error { errmsg, errnum }
    }

    pub fn errno() -> Error {
        Error::new(errno())
    }

    pub fn msg(&self) -> &str {
        &self.errmsg
    }

    pub fn num(&self) -> i32 {
        self.errnum
    }
}

impl error::Error for Error { }

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.errnum, self.errmsg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.errnum, self.errmsg)
    }
}

impl cmp::Eq for Error { }

impl cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.errnum == other.errnum
    }
}
