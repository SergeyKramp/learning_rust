use std::fs;
use std::io;

const OUTPUT_FILE_NAME: &str = "output.txt";

trait Writeable {
    fn write_all(&mut self, buffer: &[u8]) -> io::Result<()>;
}

impl Writeable for fs::File {
    fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
        self.write_all(buffer)
    }
}

fn get_user_input() -> String {
    let mut str_to_store = String::new();

    match io::stdin().read_line(&mut str_to_store) {
        Ok(_) => str_to_store,
        Err(err) => {
            println!("Could not read input: {}", err);
            String::new()
        }
    }
}

fn write_to_file(file: &mut dyn Writeable, contents: &String) {
    file.write_all(contents.as_bytes())
        .expect("Could not write to file");
}

fn main() {
    let user_input = get_user_input();
    let mut file = fs::File::create(OUTPUT_FILE_NAME).expect("Could'nt open file.");
    write_to_file(&mut file, &user_input)
}

#[cfg(test)]
mod tests {

    use super::*;

    struct MockFile {
        contents: Vec<u8>,
    }

    impl Writeable for MockFile {
        fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
            self.contents.extend_from_slice(buffer);
            Ok(())
        }
    }

    #[test]
    fn test_write_to_file() {
        // Given
        let test_string = "test";
        let contents = String::from(test_string);
        let mut mock_file = MockFile {
            contents: Vec::new(),
        };

        // When
        write_to_file(&mut mock_file, &contents);

        // Then
        assert_eq!(String::from_utf8(mock_file.contents).unwrap(), contents)
    }
}
