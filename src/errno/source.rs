//
// Created:  Tue 25 Nov 2025 02:29:09 PM PST
// Modified: Tue 25 Nov 2025 03:02:50 PM PST
//
// Copyright (C) 2025 Robert Gill <rtgill82@gmail.com>
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

use std::error::Error;
use std::ffi::FromVecWithNulError;
use std::ffi::NulError;

pub enum Source {
    FromVecWithNulError(FromVecWithNulError),
    NulError(NulError),
    None
}

impl Source {
    pub const fn error_ref(&self) -> Option<&(dyn Error + 'static)> {
	use crate::errno::source::Source::FromVecWithNulError;
	use crate::errno::source::Source::NulError;

        match &self {
            FromVecWithNulError(e) => Some(e as &dyn Error),
            NulError(e) => Some(e as &dyn Error),
            Source::None => None
        }
    }
}
