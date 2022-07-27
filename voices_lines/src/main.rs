use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut file_name: String = String::new();
    let mut placeholder: String = String::new();
    let mut loop_boolean: bool = true;

    while loop_boolean {

        println!("1 for Study Drugs");
        println!("2 for Itchy Ike");
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read line");

        // file_name.push_str(extention);

        file_name.pop();

        println!("File name: {}", file_name);

        let contents = File::open(&file_name)
            .expect("Error reading file");

        let buf_reader = BufReader::new(contents);

        for line in buf_reader.lines() {
            println!("{}", line.unwrap());
            io::stdin()
                .read_line(&mut placeholder)
                .expect("Failed to read line");
        }

        let mut repeat = "n".to_string();

        println!("Would you like to repeat (y/n)?");
        io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read line");
/*
        if repeat.pop().unwrap() == "y".to_string() {
            loop_boolean = true;
        }
        else {
            loop_boolean = false;
        }
*/
    }
    
    Ok(())
}
