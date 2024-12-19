// we can use internal scope to validate the lifetimes of the references

//fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {r}");
// }

// <'a> is the lifetime of the reference
/*
Lifetime annotations have a slightly unusual syntax:
the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types.
Most people use the name 'a for the first lifetime annotation.
We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.
&i32
&'a i32
&'a mut i32
 */

 /*
  The generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of the references of parameters
  */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



/*
    The way you need to specify the lifetimes parameters depends on what your function is doing.
    If the function returns the first parameter, the lifetime parameter for the return type needs to be the same as the lifetime parameter for the first parameter.
    If the function returns the second parameter, the lifetime parameter for the return type needs to be the same as the lifetime parameter for the second parameter.   
*/
fn returnFirst<'a>(x: &'a str, y:&str)->&'a str{
    x
}

fn returnSecond<'a>(x: & str, y:&'a str)->&'a str{
    y
}

// Generic Type Parameters, Traits Bounds, and Lifetimes Together

use std::fmt::Display;  

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}   


fn main() {
    // Generic Lifetimes in Functions

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result1 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result1);

    }

    // Lifetime Annotations in Struct Definitions

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("The first sentence is {}", i.part);
    
}
