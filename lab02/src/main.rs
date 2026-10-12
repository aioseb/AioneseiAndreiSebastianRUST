// Exercitiul 1
fn add_chars_n(s: String, chr: char, count: i32) -> String {
    let mut result: String = s;

    for _i in 0..count {
        result.push(chr);
    }

    result
}

fn test_exercise_one() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);
}

// Exercitiul 2
fn add_chars_n_reference(s: &mut String, chr: char, count: i32) {
    for _i in 0..count {
        s.push(chr);
    }
}

fn test_exercise_two() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_reference(&mut s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);
}

// Exercitiul 3
fn add_space(s: &mut String, count: i32) {
    add_chars_n_reference(s, ' ', count);
}

fn add_str(s: &mut String, added_s: String) {
    s.push_str(&added_s);
}

fn add_integer(s: &mut String, integer: i64) {
    let mut integer_mut = integer;
    let mut mirror: i64 = 0;
    let mut digits: i64 = 0;

    while integer_mut > 0 {
        mirror = mirror * 10 + (integer_mut % 10);
        integer_mut /= 10;
        digits += 1;
    }

    let mut result: String = String::from("");
    while mirror > 0 {
        let digit: u8 = (mirror % 10).try_into().unwrap();
        let digit_char: char = (b'0' + digit) as char;

        result.push(digit_char);
        mirror /= 10;
        digits -= 1;

        if digits % 3 == 0 && digits > 0 {
            result.push('_');
        }
    }

    add_str(s, result);
}

fn add_float(s: &mut String, float: f64) {
    let integer_part: i64 = float as i64;
    let decimal_mask: f64 = 10_000_000_000_000_000.0;

    // Get the decimals themselves
    let mut decimals_with_trailing = (float * decimal_mask) % decimal_mask;
    while decimals_with_trailing % 10.0 == 0.0 {
        decimals_with_trailing /= 10.0;
    }

    let decimal_part: i64 = decimals_with_trailing as i64;

    add_integer(s, integer_part);
    s.push('.');
    add_integer(s, decimal_part);
}

fn teset_exercise_three() {
    let mut s = String::from("");

    add_space(&mut s, 40);
    add_str(&mut s, String::from("I 💚\n"));

    add_space(&mut s, 40);
    add_str(&mut s, String::from("RUST.\n\n"));

    add_space(&mut s, 4);
    add_str(&mut s, String::from("Most"));
    add_space(&mut s, 12);
    add_str(&mut s, String::from("crate"));
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    s.push('\n');

    add_str(&mut s, String::from("version: "));
    add_float(&mut s, 2.0458);

    println!("{s}");
}

fn main() {
    test_exercise_one();
    test_exercise_two();
    teset_exercise_three();
}
