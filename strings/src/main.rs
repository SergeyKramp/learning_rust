use std::io;
use std::fs;
use std::io::Write;


fn get_user_input() -> String {
    let mut str_to_store = String::new();

    match io::stdin().read_line(&mut str_to_store) {
        Ok(_) => {
           str_to_store
        }
        Err(err) => {
            println!("Could not read input: {}", err);
            String::new()
        }
    }
}

fn write_to_file(file_name: &str, contents: &String) {
    match fs::File::create(file_name) {
        Ok(mut file) => match file.write_all(contents.as_bytes()){
            Ok(_) => (),
            Err(error) => println!("Could not write to file: {}", error)
        },
        Err(error) => println!("Could not create file: {}", error)
    };
}

fn main() {
    println!("Please enter a valid string:");
    let input = get_user_input();
    println!("You entered the value: {}", input);
    write_to_file("output.txt", &input)
}