use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file_1 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    let greeting_file_result_2 = File::open("hello.txt");
    // match on different errors
    let greeting_file_2 = match greeting_file_result_2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // unwarp
    let greeting_file_3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    //  expect
    let greeting_file_4 =
        File::open("hello.txt").expect("hello.txt should be included in this project");
        
}

/*
   Propagating Errors
   When a function's implementation calls something that might fail, instead of handling the error within the function itself
   you can return the error to the calling code so that it can decide what to do.
*/
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// a shortcut for propagating Errors: the ? operator
fn read_username_from_file_1() -> Result<String, io::Error> {
    // A function that returns errors to the calling code using the ? operator
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);

    // shorten more
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username);
}

/*
    Where The ? Operator Can Be Used

    The ? Operator can only be used in functions 
    If we use ? Operator in Main, it will not compile

    Luckily Main can also return a Result<(), E>

    e.g.
    fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}*/