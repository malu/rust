// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-fast compile-flags directive doesn't work for check-fast
// exec-env:RUST_LOG=conditional-debug-macro-on=4

pub fn main() {
    // exits early if println! evaluates its arguments, otherwise it
    // will hit the fail.
    println!("{:?}", { if true { return; } });

    fail!();
}
