use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io;

fn main(){
    let mut placeholder: String = String::new();
    // let mut file_name: String = String::new();

    loop {

        let mut file_name = "".to_string();
        // file_name = "".to_string();

        println!("1 for Study Drugs");
        println!("2 for Itchy Ike");
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read line");

        file_name.pop();

        let contents = File::open(&file_name)
            .expect("Error reading file");

        let buf_reader = BufReader::new(contents);

        for line in buf_reader.lines() {
            println!("{}", line.unwrap());
            io::stdin()
                .read_line(&mut placeholder)
                .expect("Failed to read line");
        }

        let mut repeat = String::new();

        println!("Would you like to quit?");
        io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read line");

        if repeat.trim() == "y".to_string() {
            break
        }
        else {
        }

    }
}
