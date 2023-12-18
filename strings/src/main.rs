use std::io;

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

fn main() {
    println!("Please enter a valid string:");
    let input = get_user_input();
    println!("You entered the value: {}", input);
}
