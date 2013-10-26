// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// i386 Linux is a 32-bit platform with 64-bit long long.

pub type c_short     = i16;
pub type c_ushort    = u16;

pub type c_int       = i32;
pub type c_uint      = u32;

pub type c_long      = i32;
pub type c_ulong     = u32;

pub type c_longlong  = i64;
pub type c_ulonglong = u64;

pub type ssize_t     = i32;
pub type size_t      = u32;


// FIXME: these should hold on all platforms
pub type intptr_t  = int;
pub type uintptr_t = uint;
