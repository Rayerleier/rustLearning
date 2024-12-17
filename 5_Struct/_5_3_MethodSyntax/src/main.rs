#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 也可以分成多个impl模块写
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Rust 没有与 -> 等价的运算符；相反，Rust 有一个称为自动引用和取消引用的功能。调用方法是 Rust 中少数具有这种行为的地方之一。
// 它的工作原理如下：当您使用 object.something() 调用方法时，Rust 会自动添加 & 、 &mut 或 * ，因此 object 匹配方法的签名。换句话说，以下内容是相同的：

/* 
p1.distance(&p2);
(&p1).distance(&p2);
 */


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!("sq is {:?}", sq);


    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}