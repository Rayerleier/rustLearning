// Lifetime Elision
/*
    The Rust language provides a set of rules called lifetime elision, which are the common cases that the compiler will infer the lifetimes for you without you having to specify them.
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
/*
    The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
     These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider,
     and if your code fits these cases, you don’t need to write the lifetimes explicitly.
        
    The rules are as follows:   
    - Each parameter that is a reference gets its own lifetime parameter.
    - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    - If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
*/

/* 
    Lifetime Annotations in Struct Definitions
    a struct hold a reference, the lifetime should be defined.
*/
struct ImportantExcerpt<'a>{
    part: &'a str,
}

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a>{
    fn level(&self)->i32{
        3   
    }
}

impl<'a> ImportantExcerpt<'a>{
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}   