use std::env;   //provides access to command-line arguments
use std::fs;    //provide file i/o operations
use std::io::{self, Write}; //provides input/output operations

fn main(){
    //parsing command-line arguments
    //env::args() returns an iterator over the command-line arguments
    //.collect() collects these arguments into a "Vec<String>"
    let args: Vec<String> = env::args().collect();

    //checking argument count
    if args.len() != 3 {
        writeln!(io::stderr(), "Usage: {} <input_file> <output_file>", args[0]).unwrap();
        std::process::exit(1);
        }
    //Ensure that the program is called with exactly three arguents: the executable name, 
    //input file and output file
    //if the count is not three, it prints the correct usage to stderr and exits with a non-zero
    //status
    

    //extraction file paths
    let input_file = &args[1];
    let output_file = &args[2];

    //reading input file

    let content = match fs::read_to_string(input_file){
        Ok(content) => content,
        Err(err) =>{
            eprintln!("Error reading input file: {}", err);
            std::process::exit(1);
        }
    };
    //converting content to uppercase
    let modified_content = content.to_uppercase();

    //writing to output file

    if let Err(err) = fs::write(output_file, modified_content){
        eprintln!("Error writing to output file: {}", err);
        std::process::exit(1);
    }

    println!("File successfully converted to uppercasae.");
}
    
