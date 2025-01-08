use std::env;
use std::process;
use _12_3_RefactoringToImproveModularityAndErrorHandling::Config;



fn main() {

    let args: Vec<String> = env::args().collect();
    // calling Config::build and handling the error
    // Using unwrap_or_else allows us to define some custom, not panic! error handling
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);  

    if let Err(e) = Config::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
