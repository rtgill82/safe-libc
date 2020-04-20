//
// Created:  Thu 16 Apr 2020 01:19:57 PM PDT
// Modified: Mon 20 Apr 2020 12:37:11 PM PDT
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

pub mod errno;
pub mod stdlib;
pub mod string;

#[cfg(target_family = "unix")]
#[doc(hidden)]
pub mod posix;
#[cfg(target_family = "unix")]
#[doc(inline)]
pub use posix::grp;
#[cfg(target_family = "unix")]
#[doc(inline)]
pub use posix::pwd;
#[cfg(target_family = "unix")]
#[doc(inline)]
pub use posix::resource;
#[cfg(target_family = "unix")]
#[doc(inline)]
pub use posix::unistd;

#[cfg(target_family = "windows")]
#[doc(hidden)]
pub mod windows;

mod util;

type VoidPtr = *mut libc::c_void;
