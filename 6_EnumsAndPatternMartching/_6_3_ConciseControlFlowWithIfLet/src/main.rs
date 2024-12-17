
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
// 如果只判断一个值，可以用这样的方法来判断，减少代码量
// 其效果相当于第一个arm
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}


let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}


fn main(){

}