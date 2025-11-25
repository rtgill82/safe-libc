//
// Created:  Thu 16 Apr 2020 01:19:12 PM PDT
// Modified: Tue 25 Nov 2025 02:21:53 PM PST
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

use std::ffi::FromVecWithNulError;
use std::ffi::NulError;
use std::{cmp,fmt};
use std::{error,result};

mod source;
use crate::errno::source::Source;
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
    errnum: i32,
    source: Source
}

impl Error {
    pub fn new(errnum: i32) -> Error {
        let source = Source::None;
        match strerror(errnum) {
            Ok(errmsg) => Error { errmsg, errnum, source },
            Err(err) => err
        }
    }

    pub fn with_msg(errnum: i32, errmsg: String) -> Error {
        let source = Source::None;
        Error { errmsg, errnum, source }
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

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.error_ref()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source {
            Source::FromVecWithNulError(e) => <FromVecWithNulError as fmt::Debug>::fmt(e, f),
            Source::NulError(e) => <NulError as fmt::Debug>::fmt(e, f),
            Source::None => write!(f, "Error {{ errmsg: \"{}\", errnum: {} }}",
                                       self.errmsg, self.errnum)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source {
            Source::FromVecWithNulError(e) => write!(f, "Error: {}", e),
            Source::NulError(e) => write!(f, "Error: {}", e),
            Source::None => write!(f, "Error {}: {}", self.errnum, self.errmsg)
        }
    }
}

impl From<FromVecWithNulError> for Error {
    fn from(value: FromVecWithNulError) -> Self {
        Error {
            errmsg: String::from("Interior NUL byte"),
            errnum: 0,
            source: Source::FromVecWithNulError(value)
        }
    }
}

impl From<NulError> for Error {
    fn from(value: NulError) -> Self {
        Error {
            errmsg: String::from("Interior NUL byte"),
            errnum: 0,
            source: Source::NulError(value)
        }
    }
}

impl cmp::Eq for Error { }

impl cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.errnum == other.errnum
    }
}
