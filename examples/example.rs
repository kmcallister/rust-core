#[no_std];

use core::vec::Vec;

#[path="../core/mod.rs"]
mod core;

#[start]
fn main(_: int, _: **u8) -> int {
    let mut xs = Vec::new();
    let mut i = 0;
    while i < 100 {
        xs.push(i);
        i += 1;
    }
    0
}
