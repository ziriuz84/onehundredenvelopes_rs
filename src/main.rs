use std::{
    fs::File,
    io::{Read, Write},
    process::exit,
};

use rand::Rng;

// A function that open a file and read it
fn read_file(filename: &str) -> String {
    let mut contents = String::new();
    if let Ok(mut f) = File::open(filename) {
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
    }
    contents
}

// A function that opens a file and append a string to it
fn write_file(filename: &str, s: &str) {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("cannot open file");
    writeln!(f, "{}", s).expect("cannot write to file");
}

// A function that counts lines in a file
fn count_lines(filename: &str) -> usize {
    let contents = read_file(filename);
    contents.lines().count()
}

// A function that takes the result of read_file and return a vector of strings
fn parse_file() -> Vec<String> {
    let mut v = Vec::new();
    let contents = read_file("extracted.txt");
    contents.lines().for_each(|line| v.push(line.to_string()));
    v
}

fn is_included_in_vec(s: &str, v: &Vec<String>) -> bool {
    v.iter().any(|x| x == s)
}

// A funcion that extracts a number between 1 and 100 and returns it as a string
fn extract_number(min: i8, max: i8) -> String {
    let mut rng = rand::rng();
    let extracted = parse_file();
    let mut num = rng.random_range(min..(max + 1));
    while is_included_in_vec(&num.to_string(), &extracted) {
        num = rng.random_range(min..(max + 1));
    }
    num.to_string()
}

fn main() {
    if count_lines("extracted.txt") == 100 {
        println!("All numbers have been extracted");
        // exit the program
        exit(1);
    }
    println!("Select an option");
    println!("1. Generate a random number between 1 and 49");
    println!("2. Generate a random number between 50 and 100");
    let choice = std::io::stdin().lines().next().unwrap().unwrap();
    if choice != "1" && choice != "2" {
        println!("Invalid choice");
    }
    let mut number = String::new();
    if choice == "1" {
        number = extract_number(1, 49);
    }
    if choice == "2" {
        number = extract_number(50, 100);
    }
    println!("Number: {}", number);
    write_file("extracted.txt", &number);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_read_file() {
        let filename = "test_rw.txt";
        let content = "hello world";
        write_file(filename, content);
        let read_content = read_file(filename);
        assert_eq!(read_content.trim(), content);
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_count_lines() {
        let filename = "test_count.txt";
        write_file(filename, "line1");
        write_file(filename, "line2");
        assert_eq!(count_lines(filename), 2);
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_is_included_in_vec() {
        let vec = vec!["a".to_string(), "b".to_string()];
        assert!(is_included_in_vec("a", &vec));
        assert!(!is_included_in_vec("c", &vec));
    }

    #[test]
    fn test_parse_file() {
        let filename = "extracted.txt";
        write_file(filename, "1");
        write_file(filename, "2");
        let vec = parse_file();
        assert_eq!(vec, vec!["1".to_string(), "2".to_string()]);
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_extract_number_logic() {
        let filename = "extracted.txt";
        // Create a file with all numbers from 1 to 48
        for i in 1..49 {
            write_file(filename, &i.to_string());
        }
        // The only number that can be extracted is 49
        let number = extract_number(1, 49);
        assert_eq!(number, "49");
        fs::remove_file(filename).unwrap();
    }
}