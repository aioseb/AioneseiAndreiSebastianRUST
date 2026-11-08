use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use std::{fs, io};

fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn problem_one() -> Result<(), io::Error> {
    let content: String = read_file("text_files/problem_one.txt")?;
    let lines = content.lines();

    let mut max_bytes: usize = 0;
    let mut max_length: usize = 0;
    let mut max_str_by_bytes: &str = "";
    let mut max_str_by_length: &str = "";

    for line in lines {
        if max_bytes < line.len() {
            max_bytes = line.len();
            max_str_by_bytes = line;
        }

        // Calculate the length by char
        let chars: usize = line.chars().count();
        // for _ in line.chars() {
        //     chars += 1;
        // }

        if max_length < chars {
            max_length = chars;
            max_str_by_length = line;
        }
    }

    println!("Longest line (bytes): {max_str_by_bytes}");
    println!("Longest line (chars): {max_str_by_length}");

    Ok(())
}

enum CipherError {
    Rot13,
}

// Applies rot13 cipher over given string and returns it
fn rot13_cipher(text: &str) -> Result<String, CipherError> {
    let mut encrypted: String = String::from("");

    for chr in text.chars() {
        if !chr.is_ascii() {
            return Err(CipherError::Rot13);
        }

        let result: char;

        // Encrypt the character if it's a letter
        if chr.is_alphabetic() {
            if (chr as u8 + 13) > b'z' || ((chr as u8 + 13) > b'Z' && chr < 'A') {
                result = (chr as u8 - 13) as char;
            } else {
                result = (chr as u8 + 13) as char;
            }

            encrypted.push(result);
        } else {
            encrypted.push(chr);
        }
    }

    Ok(encrypted)
}

fn problem_two() -> Result<(), CipherError> {
    let text1: &str = "rust";
    let text2: &str = "strings are fun";
    let text3: &str = "🎁🎶🎉👀🎈🎃🍕☕🍉";

    println!("{}", rot13_cipher(text1)?);
    println!("{}", rot13_cipher(text2)?);
    println!("{}", rot13_cipher(text3)?);

    Ok(())
}

fn problem_three() -> Result<(), io::Error> {
    let content: String = read_file("text_files/problem_three.txt")?;
    let lines = content.lines();

    for line in lines {
        // Replace every abbreviation
        let mut new_line: String;

        new_line = line.replace("ptr", "pentru");
        new_line = new_line.replace("pt", "pentru");
        new_line = new_line.replace("dl", "domnul");
        new_line = new_line.replace("dna", "doamna");

        println!("{new_line}");
    }

    Ok(())
}

fn problem_four() -> Result<(), io::Error> {
    let content: String = read_file("text_files/hosts.txt")?;
    let lines = content.lines();

    for line in lines {
        // Ignore lines that have a leading '#'
        if line.starts_with("#") {
            continue;
        }

        let mut dns = line.split_whitespace();
        let ip = dns.next().unwrap_or("0.0.0.0");
        let hostname = dns.next().unwrap_or("no.hostname");

        if ip == "0.0.0.0" {
            continue;
        }

        println!("{hostname} => {ip}");
    }

    Ok(())
}

fn create_file() -> Result<(), std::io::Error> {
    let kb: u64 = 1_024;
    // let mb: u64 = 1_024 * kb;
    // let gb: u64 = 1_024 * mb;

    // Create and open empty file
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open("text_files/bonus.txt")?;

    // Write 1 GB in it
    for _ in 0..kb {
        write!(f, "a")?;
    }

    let content: String = read_file("text_files/bonus.txt")?;

    match rot13_cipher(content.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(std::io::Error::other("Error in cipher")),
    }
}

fn bonus() -> Result<(), io::Error> {
    let start = Instant::now();
    create_file()?;
    println!("{:?}", start.elapsed());

    Ok(())
}

fn main() {
    match problem_one() {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }

    match problem_two() {
        Ok(()) => {}
        Err(_) => println!("Text couldn't be encrypted"),
    }

    match problem_three() {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }

    match problem_four() {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }

    match bonus() {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }
}
