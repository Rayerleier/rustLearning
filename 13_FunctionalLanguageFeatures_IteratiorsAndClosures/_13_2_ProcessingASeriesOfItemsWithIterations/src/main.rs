// in rust, iterators are lazy, meaning they don't compute their values until you ask for them.
// this means you can chain together many iterators and only compute values when you need them.
// this is useful for processing data in a way that is efficient and flexible.

// All iterators implement a trait named Iterator that is defined in the standard library.

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    v1.iter().map(|x| x + 1);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
