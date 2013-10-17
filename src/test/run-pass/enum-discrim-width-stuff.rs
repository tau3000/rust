// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;

pub fn main() {
    enum E { V = 0x1717171717171717 }
    static C: E = V;
    let expected: u64 = if mem::size_of::<uint>() < 8 {
        0x17171717
    } else {
        0x1717171717171717
    };
    assert_eq!(expected, V as u64);
    assert_eq!(expected, C as u64);
    assert_eq!(format!("{:?}", V), ~"V");
    assert_eq!(format!("{:?}", C), ~"V");
}
