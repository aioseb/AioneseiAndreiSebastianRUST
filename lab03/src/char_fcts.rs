// Exercise 4

pub enum CharError {
    Ascii,
    Digit,
    Hex,
    Letter,
    Printable,
}

pub fn to_uppercase(chr: char) -> Result<char, CharError> {
    if !chr.is_alphabetic() {
        return Err(CharError::Letter);
    }
    Ok(chr.to_ascii_uppercase())
}

pub fn to_lowercase(chr: char) -> Result<char, CharError> {
    if !chr.is_alphabetic() {
        return Err(CharError::Letter);
    }
    Ok(chr.to_ascii_lowercase())
}

pub fn print_char(chr: char) -> Result<(), CharError> {
    if chr.is_ascii_control() {
        return Err(CharError::Printable);
    }
    print!("{chr}");
    Ok(())
}

pub fn char_to_number(chr: char) -> Result<u8, CharError> {
    if !chr.is_ascii() {
        return Err(CharError::Ascii);
    }

    if !chr.is_ascii_digit() {
        return Err(CharError::Digit);
    }

    Ok(chr.to_digit(10).unwrap_or(10) as u8)
}

pub fn char_to_number_hex(chr: char) -> Result<u8, CharError> {
    if !chr.is_ascii() {
        return Err(CharError::Ascii);
    }

    if !chr.is_ascii_hexdigit() {
        return Err(CharError::Hex);
    }

    Ok(chr.to_digit(16).unwrap_or(16) as u8)
}

pub fn print_error(error: CharError) {
    match error {
        CharError::Ascii => println!("Character is not an ASCII character!"),
        CharError::Digit => println!("Character is not a digit!"),
        CharError::Hex => println!("Character is not a hex digit!"),
        CharError::Letter => println!("Character is not a letter!"),
        CharError::Printable => println!("Character is not printable!"),
    }
}
