use std::collections::HashMap;

fn main(){
    let mut v:Vec<i32> = Vec::new();

    v = vec![1,2,3,1,3,4,5,1,2,3,5,1,1,1];

    v.sort();
    let median = v.get(v.len()/2);
    match median{
        Some(median) => println!("中位数是:{}", median),
        None => println!("None"),
    }

    let mut hash = HashMap::new();
    for i in &v {
        let count = hash.entry(*i).or_insert(0);
        *count += 1;
    }



    println!("{hash:?}");
    let mut mode = 0;
    let mut max = 0;
    for (key, value ) in &hash{
        if *value > max{
            max = *value;
            mode = *key;
        }
    }
    println!("众数是:{}", mode);
}