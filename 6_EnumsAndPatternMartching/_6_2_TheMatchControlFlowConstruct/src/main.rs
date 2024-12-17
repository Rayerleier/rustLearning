
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// value_in_cents(Coin::Quarter(UsState::Alaska))
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let dice_roll = 9;
// match是按照顺序来的，如果把other放在前面，则任何的都不会匹配
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
// _ 是一种特殊模式，可以匹配任何值并且不绑定到那个值。
//这告诉 Rust 我们不会使用该值，因此 Rust 不会警告我们有关未使用的变量。
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
fn reroll() {}
// 空元组表示什么都不会发生
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}



fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    }
