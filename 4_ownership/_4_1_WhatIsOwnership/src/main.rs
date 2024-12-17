fn main() {
    let s = "hello";
    // String是管理在堆上的数据，因此能够存储编译我们未知的大量文本
    let s = String::from("hello"); // 不能被改变
    let mut s = String::from("hello"); // 可以被改变
    s.push_str(", world!");
    println!("{}", s)
    //有一个自然的点，我们可以将 String 所需的内存返回给分配器：当 s 超出范围时。当变量超出范围时，Rust 会为我们调用一个特殊的函数。这个函数称为 drop ， String 的作者可以在其中放置返回内存的代码。 Rust 在右大括号处自动调用 drop 。

    let s1 = String::from("hello"); // 数据存放在栈中，还有内存的指针，长度和容量
    // 在s2复制s1的时候，会复制内容的指针，长度，和容量，不会复制指针中内存的内容
    let s2 = s1;
    // 此时两个数据指针都指向同一位置，在超出范围后两个变量释放相同的内存
    // 这会殷勤双重释放错误。
    // 为了确保内存安全，在 let s2 = s1; 行之后，Rust 认为 s1 不再有效。因此，当 s1 超出范围时，Rust 不需要释放任何内容。查看创建 s2 后尝试使用 s1 时会发生什么情况；它不会工作：
    // println!("{}, world!", s1);
    //此外，这还暗示着一个设计选择：Rust 永远不会自动创建数据的“深层”副本。因此，就运行时性能而言，任何自动复制都可以被认为是廉价的。


     
    // 如果我们确实想要深度复制 String 的堆数据，而不仅仅是堆栈数据，我们可以使用一个名为 clone 的通用方法。
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    let x = 5;
    let y = x;
    // 在这一部分是有效的，为什么？因为编译器会将已知大小的变量放到堆中
    // 这代表没有理由阻止x有效。这里的深浅拷贝没有任何区别。
    println!("x = {}, y = {}", x, y);

    // Rust 有一个特殊的注释，称为 Copy 特征，我们可以将其放置在存储在堆栈中的类型上，就像整数一样（我们将在第 10 章中详细讨论特征）。
    // 如果某个类型实现了 Copy 特征，则使用它的变量不会移动，而是会被简单地复制，从而使它们在分配给另一个变量后仍然有效。
    // 如果类型或其任何部分实现了 Drop 特征，Rust 不会让我们用 Copy 注释类型。
    // 如果当值超出范围时类型需要发生一些特殊的事情，并且我们向该类型添加 Copy 注释，我们将收到编译时错误。
    // 要了解如何将 Copy 注释添加到您的类型以实现特征，请参阅附录 C 中的“可派生特征”。
    /*
        哪些实现了copy特征
        所有整数类型，例如 u32 。
        布尔类型 bool ，其值为 true 和 false 。
        所有浮点类型，例如 f64 。
        字符类型 char 。
        元组，如果它们仅包含也实现 Copy 的类型。例如， (i32, i32) 实现 Copy ，但 (i32, String) 没有实现。

     */

     let s = String::from("hello");  // s comes into scope

     takes_ownership(s);             // s's value moves into the function...
                                     // ... and so is no longer valid here
 
     let x = 5;                      // x comes into scope
 
     makes_copy(x);                  // x would move into the function,
                                     // but i32 is Copy, so it's okay to still
                                     // use x afterward
 
    //返回值也可以转移所有权。
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3


    // 返回多个值
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens