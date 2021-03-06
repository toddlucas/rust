// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:specialization_cross_crate_defaults.rs

#![feature(specialization)]

extern crate specialization_cross_crate_defaults;

use specialization_cross_crate_defaults::*;

struct LocalDefault;
struct LocalOverride;

impl Foo for LocalDefault {}

impl Foo for LocalOverride {
    fn foo(&self) -> bool { true }
}

fn test_foo() {
    assert!(0i8.foo() == false);
    assert!(0i32.foo() == false);
    assert!(0i64.foo() == true);

    assert!(LocalDefault.foo() == false);
    assert!(LocalOverride.foo() == true);
}

fn test_bar() {
    assert!(0u8.bar() == 0);
    assert!(0i32.bar() == 1);
    assert!("hello".bar() == 0);
    assert!(vec![()].bar() == 2);
    assert!(vec![0i32].bar() == 2);
    assert!(vec![0i64].bar() == 3);
}

fn main() {
    test_foo();
    test_bar();
}
