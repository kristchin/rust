// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unreachable_code)]
// compile-flags: --edition 2018

#![feature(try_blocks)]

fn main() {
    let mut a = 0;
    let () = {
        let _: Result<(), ()> = try {
            let _ = Err(())?;
            return
        };
        a += 1;
    };
    a += 2;
    assert_eq!(a, 3);
}
