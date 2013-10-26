/*! Draws a maze in the style of 10print.org.

    A standalone Linux binary: no Rust stdlib, no libc.

        rustc -O --lib --emit-llvm examples/linux_maze_nolibc.rs && \
        clang -O2 -nostdlib -static -o linux_maze_nolibc \
            examples/linux_maze_nolibc.bc cbits/libc_intrinsics.c && \
        ./linux_maze_nolibc

    Building with rustc --lib solves several problems.  We don't get a C main
    and we can define our own _start without it being removed as dead code.

    libc_intrinsics.c is not needed at all optimization levels.
*/

#[link(name = "maze", vers = "0.0")];

#[no_std];
#[feature(asm, macro_rules)];
#[allow(ctypes, cstack, unused_mut)];

use core::{mem, slice};
use core::platform::c_types::c_ushort;

#[path="../core/mod.rs"]
mod core;

#[path="../core/macros.rs"]
mod macros;

/// The linker looks for this symbol and will set the ELF entry point to here.
/// It's 'pub' so that it doesn't get removed as dead code.
/// FIXME: should be unsafe extern "C" fn; see Rust #10025
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        syscall!(EXIT, main());
    }

    // If we return we will probably segfault, since there are no more stack
    // frames below _start.
}

/// Access a static string's buffer and length.
fn static_str(x: &'static str) -> slice::Slice<u8> {
    unsafe { mem::transmute(x) }
}

/// The maze is drawn by choosing randomly between these characters (U+2571, U+2572).
static lines: [&'static str, ..2] = ["╱", "╲"];

/// Linux terminal size struct; see tty_ioctl(4).
struct winsize {
    ws_row:    c_ushort,
    ws_col:    c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}

// System call constants.
// These are the same on 32- and 64-bit x86 Linux.
static STDOUT_FILENO: uint = 1;
static O_RDONLY:      uint = 0;
static TIOCGWINSZ:    uint = 0x5413;

/// Get the size of the terminal in characters.
unsafe fn term_size() -> uint {
    let mut sz: winsize = mem::uninit();
    syscall!(IOCTL, STDOUT_FILENO, TIOCGWINSZ, &mut sz as *mut winsize);
    (sz.ws_row as uint) * (sz.ws_col as uint)
}

/// Draw a maze!
unsafe fn main() -> uint {
    let urandom_fd = syscall!(OPEN, static_str("/dev/urandom\0").data, O_RDONLY);

    // We may not have a libc but we still have closures!
    let rand8 = || {
        let mut buf: u8 = 0;
        syscall!(READ, urandom_fd, &mut buf as *mut u8, 1);
        buf
    };

    let mut n = term_size();
    while n > 0 {
        let line = static_str(lines[rand8() % 2]);
        syscall!(WRITE, STDOUT_FILENO, line.data, line.len);
        n -= 1;
    }

    0
}
