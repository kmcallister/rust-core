// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[inline(always)]
pub unsafe fn syscall0(n: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}

#[inline(always)]
pub unsafe fn syscall1(n: uint, a1: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}

#[inline(always)]
pub unsafe fn syscall2(n: uint, a1: uint, a2: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}

#[inline(always)]
pub unsafe fn syscall3(n: uint, a1: uint, a2: uint, a3: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}

#[inline(always)]
pub unsafe fn syscall4(n: uint, a1: uint, a2: uint, a3: uint, a4: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3), "{r10}"(a4)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}

#[inline(always)]
pub unsafe fn syscall5(n: uint, a1: uint, a2: uint, a3: uint, a4: uint, a5: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3), "{r10}"(a4), "{r8}"(a5)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}

#[inline(always)]
pub unsafe fn syscall6(n: uint, a1: uint, a2: uint, a3: uint, a4: uint, a5: uint, a6: uint) -> uint {
    let mut ret : uint;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3), "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                   : "rcx", "r11", "memory"
                   : "volatile");
    return ret;
}
