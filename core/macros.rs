// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_escape];

/// Useful macros.  This module isn't attached anywhere within core::, since
/// macros are only visible for things after/below them in the AST.
/// If you need this module you should attach it yourself, e.g.
///
///     #[path="core/macros.rs"]
///     mod macros;

macro_rules! syscall(
    ($nr:ident)
        => ( ::core::platform::syscall::raw::syscall0(
                ::core::platform::syscall::number::$nr) );

    ($nr:ident, $a1:expr)
        => ( ::core::platform::syscall::raw::syscall1(
                ::core::platform::syscall::number::$nr,
                $a1 as uint) );

    ($nr:ident, $a1:expr, $a2:expr)
        => ( ::core::platform::syscall::raw::syscall2(
                ::core::platform::syscall::number::$nr,
                $a1 as uint, $a2 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr)
        => ( ::core::platform::syscall::raw::syscall3(
                ::core::platform::syscall::number::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::core::platform::syscall::raw::syscall4(
                ::core::platform::syscall::number::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint,
                $a4 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::core::platform::syscall::raw::syscall5(
                ::core::platform::syscall::number::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint,
                $a4 as uint, $a5 as uint) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::core::platform::syscall::raw::syscall6(
                ::core::platform::syscall::number::$nr,
                $a1 as uint, $a2 as uint, $a3 as uint,
                $a4 as uint, $a5 as uint, $a6 as uint) );
)
