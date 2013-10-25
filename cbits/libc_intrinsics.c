// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <stddef.h>

// This file provides standalone implementations of libc functions which Rust
// code sometimes calls implicitly.
//
// These functions aren't fast or well-tested.  They're just here so we can get
// standalone code up and running without licensing issues from pulling in
// third party libc code.
//
// This file isn't integrated into the build; just pass it to clang in your
// final compile/link step.
//
// It would be fun to write these in Rust but we'd have to prevent the Rust
// compiler from inserting recursive calls!  There are other problems as well,
// such as Rust #10025.

void *memset(void *s, int c, size_t n) {
    unsigned char *buf = (unsigned char *) s;
    unsigned char fill = (unsigned char)   c;

    while (n--) {
        *buf++ = fill;
    }

    return s;
}

void *memmove(void *dest, const void *src, size_t n) {
    unsigned char *cdest = (unsigned char *) dest;
    unsigned char *csrc  = (unsigned char *) src;

    if (csrc < cdest) {
        // copy from end
        cdest += n;
        csrc  += n;

        while (n--) {
            *(--cdest) = *(--csrc);
        }
    } else {
        // copy from beginning
        while (n--) {
            *(cdest++) = *(csrc++);
        }
    }

    return dest;
}

void *memcpy(void *dest, const void *src, size_t n) {
    return memmove(dest, src, n);
}
