use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
//就像向量一样，哈希映射将其数据存储在堆上。
//此 HashMap 具有 String 类型的键和 i32 类型的值。
//与向量一样，哈希映射是同构的：所有键必须具有相同的类型，并且所有值必须具有相同的类型。


let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
// 在这里， score 将具有与蓝队关联的值，结果将为 10 。 
// get 方法返回一个 Option<&V> ；如果哈希映射中该键没有值， get 将返回 None 。
// 该程序通过调用 copied 来处理 Option 来获取 Option<i32> 而不是 Option<&i32> ，然后 unwrap_or 来如果 scores 没有该密钥的条目，则将 score 设置为零。

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);

//哈希映射有一个特殊的 API，称为 entry ，它将您要检查的密钥作为参数。 
// entry 方法的返回值是一个名为 Entry 的枚举，它表示一个可能存在也可能不存在的值。假设我们要检查黄队的密钥是否有与之关联的值。
// 如果没有，我们要插入值 50，蓝队也是如此。使用 entry API，代码如清单 8-24 所示。

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");



let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
