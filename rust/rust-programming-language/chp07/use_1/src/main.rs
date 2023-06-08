use std::collections::HashMap;
// use std::fmt;
// use std::io;

use std::fmt::Result;
use std::io::Result as IoResult;

// use std::cmp::Ordering;
// use std::io;

// Nested paths
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

use std::io::{self, Write};

// "*"" glob operator brings all public items defined into scope
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

fn function1() -> Result {}
fn function2() -> IoResult<()> {}
