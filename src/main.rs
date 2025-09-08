use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process::exit,
};

use rand::Rng;

// A function that reads extracted numbers from file and returns a HashSet for O(1) lookups
fn read_extracted_numbers(filename: &str) -> Result<HashSet<String>, io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // File doesn't exist yet, return empty HashSet
            return Ok(HashSet::new());
        }
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);
    let mut numbers = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            numbers.insert(line.trim().to_string());
        }
    }
    Ok(numbers)
}

// A function that opens a file and append a string to it
fn write_file(filename: &str, s: &str) -> Result<(), io::Error> {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    writeln!(f, "{}", s)?;
    Ok(())
}

// A function that counts extracted numbers efficiently
fn count_extracted_numbers(filename: &str) -> Result<usize, io::Error> {
    let numbers = read_extracted_numbers(filename)?;
    Ok(numbers.len())
}

// A function that extracts a number between min and max that hasn't been extracted yet
fn extract_number(min: i8, max: i8) -> Result<String, io::Error> {
    let mut rng = rand::rng();
    let extracted = read_extracted_numbers("extracted.txt")?;

    // Generate numbers until we find one that hasn't been extracted
    loop {
        let num = rng.random_range(min..=max);
        let num_str = num.to_string();
        if !extracted.contains(&num_str) {
            return Ok(num_str);
        }
    }
}

fn main() {
    // Check if all numbers have been extracted
    match count_extracted_numbers("extracted.txt") {
        Ok(count) if count >= 100 => {
            println!("All numbers have been extracted");
            exit(0);
        }
        Err(e) => {
            eprintln!("Error reading extracted numbers: {}", e);
            exit(1);
        }
        _ => {} // Continue if count < 100
    }

    println!("Select an option:");
    println!("1. Generate a random number between 1 and 49");
    println!("2. Generate a random number between 50 and 100");
    println!("3. Generate a random number between 1 and 100");

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", e);
        exit(1);
    }

    let choice = input.trim();
    let number = match choice {
        "1" => extract_number(1, 49),
        "2" => extract_number(50, 100),
        "3" => extract_number(1, 100),
        _ => {
            println!("Invalid choice. Please select 1 or 2.");
            return;
        }
    };

    match number {
        Ok(num) => {
            println!("Number: {}", num);
            if let Err(e) = write_file("extracted.txt", &num) {
                eprintln!("Error writing to file: {}", e);
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error extracting number: {}", e);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_read_extracted_numbers() {
        let filename = "test_rw.txt";
        let content = "hello world";
        write_file(filename, content).unwrap();
        let numbers = read_extracted_numbers(filename).unwrap();
        assert!(numbers.contains("hello world"));
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_count_extracted_numbers() {
        let filename = "test_count.txt";
        write_file(filename, "line1").unwrap();
        write_file(filename, "line2").unwrap();
        assert_eq!(count_extracted_numbers(filename).unwrap(), 2);
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_read_extracted_numbers() {
        let filename = "test_read.txt";
        write_file(filename, "1").unwrap();
        write_file(filename, "2").unwrap();
        let numbers = read_extracted_numbers(filename).unwrap();
        assert!(numbers.contains("1"));
        assert!(numbers.contains("2"));
        assert_eq!(numbers.len(), 2);
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_extract_number_logic() {
        let filename = "extracted.txt";
        // Create a file with all numbers from 1 to 48
        for i in 1..49 {
            write_file(filename, &i.to_string()).unwrap();
        }
        // The only number that can be extracted is 49
        let number = extract_number(1, 49).unwrap();
        assert_eq!(number, "49");
        fs::remove_file(filename).unwrap();
    }
}
