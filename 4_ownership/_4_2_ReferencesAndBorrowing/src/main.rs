fn main() {
    let s1 = String::from("hello");

    // 引用类型表示你可以引用它的值而不获取所有权。
    // 引用符号 &  取消引用 *
    // &s1 表示创建一个引用s1的值，由于是新创建了一个值，自然不会用掉s1的所有权

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    //我们将创建引用的操作称为借用。
    //那么，如果我们尝试修改借用的东西会发生什么？
    //这不起作用！
    change(&mut s);
    //正如变量在默认情况下是不可变的一样，引用也是如此。我们不允许修改我们所引用的内容。
    // 调整成可变引用就可以了
    // 可变引用有一个很大的限制：如果您有一个对某个值的可变引用，则不能有对该值的其他引用。尝试创建两个对 s 的可变引用的代码将失败：

    /* 
    具有此限制的好处是 Rust 可以防止编译时的数据竞争。
    数据竞争与竞争条件类似，当发生以下三种行为时就会发生：
    两个或多个指针同时访问相同的数据。
    至少有一个指针用于写入数据。
    没有使用任何机制来同步对数据的访问。
    */
    let mut s = String::from("hello");
    // 通过限制范围来允许多个可变引用，但不能同时引用
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //Rust 强制执行类似的规则来组合可变和不可变引用。此代码会导致错误：
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    //哇！当我们拥有相同值的不可变引用时，我们也不能拥有可变引用。
    println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);



    /*
    在带有指针的语言中，通过释放一些内存同时保留指向该内存的指针，
    很容易错误地创建一个悬空指针（引用内存中可能已分配给其他人的位置的指针）。
    相比之下，在 Rust 中，编译器保证引用永远不会是悬空引用：
    如果您引用了某些数据，编译器将确保数据不会在数据引用超出范围之前超出范围。
     */
    let reference_to_nothing = dangle();
    // 该报错涉及生命周期概念。第十章讨论
}   

fn dangle() -> &String {
    let s = String::from("hello");

    &s  // 返回s的引用，但是s是在dangle中的作用域，离开dangle，s就被释放了，因此引用是空引用
    // 这里的解决方案是直接返回String
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn calculate_length(s: &String) -> usize {
    s.len()
}