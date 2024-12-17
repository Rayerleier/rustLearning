struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


//没有任何字段的类似单元的结构
struct AlwaysEqual;



fn main() {
    //请注意，整个实例必须是可变的； Rust 不允许我们仅将某些字段标记为可变。
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // struct update语法
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    /*
    请注意，结构更新语法使用 = 就像赋值一样；这是因为它移动数据，
    正如我们在“与移动交互的变量和数据”部分中看到的那样。
    在这个例子中，我们在创建 user2 之后就不能再将 user1 作为一个整体使用了，因为username > 已移至 user2 中。
    如果我们为 email 和 username 赋予 user2 新的 String 值，因此仅使用 active 和 sign_in_count 值来自 user1 ，那么 user1 在创建 user2 后仍然有效。
    active 和 sign_in_count 都是实现 Copy 特征的类型，因此我们在“仅堆栈数据：复制”部分中讨论的行为将适用。
     */

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // 因为名称相同，可以直接简写
        email,
        sign_in_count: 1,
    }
}
