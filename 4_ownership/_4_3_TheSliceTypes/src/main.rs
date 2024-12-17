fn main() {
    /*
    切片允许您引用集合中连续的元素序列，而不是整个集合。
    切片是一种引用，因此它没有所有权。
     */
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("{}", word); // 这样获取word的值是5，与字符串没有关联
                          //如果此时要获得第二个单词就会变得麻烦，因此导入切片
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");
    // 字符串切片
    let hello = &s[0..5];
    let world = &s[6..11];
    //这两句是相等的
    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..len];
    let slice = &s[3..];

    // 字符串切片作为参数
    // fn first_word(s: &str) -> &str {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //逐个遍历String并检查是否为空格

    // 用iter方法创建一个迭代器
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
