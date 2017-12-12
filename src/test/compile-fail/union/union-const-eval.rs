// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

union U {
    a: usize,
    b: usize,
}

const C: U = U { a: 10 };

fn main() {
    unsafe {
        let a: [u8; C.a]; // OK
        let b: [u8; C.b]; //~ ERROR constant evaluation error
                          //~^ NOTE nonexistent struct field
                          //~| WARNING constant evaluation error
                          //~| NOTE on by default
    }
}
