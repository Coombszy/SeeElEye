use std::{fs::File, io::{BufReader, BufRead}};

fn main() {

    let file_path = "./static/favicon.py";

    println!("Files in path {}", file_path);


    let file = File::open(file_path).expect("Should be able to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }




}
