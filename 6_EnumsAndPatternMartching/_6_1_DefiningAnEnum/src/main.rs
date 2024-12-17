enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


// 标准库中怎么做的
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


//一个枚举变体里多了好几种类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// 枚举类型还能实现imple

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();


// rust可以堆不存在的值这个概念进行编码
// 标准库代码如下
enum Option<T> {
    None,
    Some(T),
}
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
/*
消除错误假设非空值的风险可以帮助您对代码更有信心。
为了获得可能为 null 的值，您必须通过设置该值的类型 Option<T> 来显式选择加入。
然后，当您使用该值时，需要显式处理该值为 null 的情况。
只要值的类型不是 Option<T> ，您就可以安全地假设该值不为 null。
这是 Rust 的一个深思熟虑的设计决策，旨在限制 null 的普遍性并提高 Rust 代码的安全性。
 */

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
