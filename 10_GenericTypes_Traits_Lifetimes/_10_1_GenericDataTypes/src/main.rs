fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let both_integer = Point{x: 5, y: 10};
    let both_float = Point{x: 1.0, y: 4.0};
    let integer_and_float = Point{x: 5, y: 4.0};    

    let p = Point{x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p1 = Point{x: 5, y: 10.4};
    let p2 = Point{x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x(), p3.y());

}
