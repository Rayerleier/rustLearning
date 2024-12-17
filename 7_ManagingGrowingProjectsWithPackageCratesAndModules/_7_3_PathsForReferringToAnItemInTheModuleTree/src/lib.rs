mod front_of_house {
    pub mod hosting {
       pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}


// super返回上级的调用，实际上是在域外
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

//我们还可以使用 pub 将结构和枚举指定为公共结构和枚举
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
/*
由于 back_of_house::Breakfast 结构中的 toast 字段是公共的，
因此在 eat_at_restaurant 中我们可以使用点表示法写入和读取 toast 字段。
请注意，我们不能在 eat_at_restaurant 中使用 seasonal_fruit 字段，
因为 seasonal_fruit 是私有的。尝试取消注释修改 seasonal_fruit 字段值的行，看看会出现什么错误！

 */

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// 如果我们公开一个枚举，那么它的所有变体都是公开的
// 枚举默认都是公开的，结构体默认是私有的。
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}