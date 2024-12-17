fn main() {
    let v: Vec<i32> = Vec::new();
    // 更常见的用法是用向量宏创建一个自动识别的。
    let v = vec![1,2,3];

    let mut v = Vec::new();
    // 添加元素
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 读取元素
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");
    // 用get方法时会获得一个Option<&T>
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /*
    为什么对第一个元素的引用应该关心向量末尾的变化？
    此错误是由于向量的工作方式造成的：
    因为向量将值放在内存中彼此相邻的位置，
    因此在向量末尾添加新元素可能需要分配新内存并将旧元素复制到新空间（如果有）没有足够的空间将当前存储向量的所有元素彼此相邻。
    在这种情况下，对第一个元素的引用将指向已释放的内存。
    借用规则可以防止程序陷入这种情况。
     */

    // 迭代向量
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 取消引用，来更改其值
    }

    // 利用enum来使向量可以存储不同类型的值
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here


}
