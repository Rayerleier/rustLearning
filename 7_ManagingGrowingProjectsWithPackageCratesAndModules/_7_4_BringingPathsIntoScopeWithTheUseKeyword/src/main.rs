mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

//使用 use 将 add_to_waitlist 函数引入作用域，这是不惯用的
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//另一方面，当使用 use 引入结构、枚举和其他项目时，指定完整路径是惯用的。
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 使用as重命名，通常情况下还是标记作用域来区分
//  fmt::Result
//  io::Result
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
//使用 pub use 为新作用域中的任何代码提供可用的名称

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

use std::cmp::Ordering;
use std::io;
// 等同于
use std::{cmp::Ordering, io};


use std::io;
use std::io::Write;
//等同于
use std::io::{self, Write};


use std::collections::*;
