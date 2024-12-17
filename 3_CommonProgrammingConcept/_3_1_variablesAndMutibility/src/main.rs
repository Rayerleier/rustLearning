
fn main() {
    let x = 5; // 不可变变量
    println!("The value of x is : {x} ");
    let mut y = 5;  //可变变量
    y = 7;
    println!("y is : {y}");
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3; //常量
    let mut spaces = "   ";
    spaces = spaces.len(); //会报类型错误
    //已经声明的变量不能做隐式变量转换


}
