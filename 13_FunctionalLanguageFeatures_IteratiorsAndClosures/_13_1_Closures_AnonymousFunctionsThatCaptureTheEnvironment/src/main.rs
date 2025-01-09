use std::thread;
use std::time::Duration;    

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // We've passed a closure to unwrap_or_else. Closure expression `|| self.most_stocked()` is an anonymous function that captures the environment.
        user_preference.unwrap_or_else(|| self.most_stocked())
        // Closures doesn't require you to annotate the types of the parameters of the return value like fn.
        // Defining this interface rigidly is imporatant for ensuring that everyone agrees on what types of a function uses and returns.
        // Closures, on the oother hand, aren't used in an exposed interface, they are stored in variables and used without  naming them and exposing them to users of our library.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}   

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    //The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String.
    // Those types are then locked into the closure in example_closure, and we get a type error when we next try to use a different type with the same closure.
    expensive_closure(5);
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x: u32| x + 1;

    
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // if you want to force the closure to take ownership of the values it uses in the environment, you can use the move keyword.   

    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();

    // let mut borrows_mutably = || list.push(7);

    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    let mut list2 = vec![Rectangle { width: 10, height: 1 }];

    list2.push(Rectangle { width: 3, height: 5 });
    list2.push(Rectangle { width: 7, height: 4 });

    list2.sort_by_key(|r| r.width);
    // the reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice.
    // The closure |r| r.width dosen't capture, mutate or move out anything from its environment, so it meets the trait bound requirements.
    println!("{:?}", list2);

    let mut list3 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 4 },
    ];
    let mut nums_sort_operations = 0;
    list3.sort_by_key(|r| {
        nums_sort_operations += 1;
        r.width
    });
    println!("{:?}, sorted in {nums_sort_operations} operations", list3);

}
