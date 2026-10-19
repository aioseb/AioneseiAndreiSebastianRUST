struct Note {
    title: String,
    body: String,
}

// ---------- Part 1: borrow to read (&str) ----------

// How many times does `c` appear in `s`?
fn count_char(s: &str, c: char) -> usize {
    let mut count: usize = 0;

    for str_char in s.chars() {
        if str_char == c {
            count += 1;
        }
    }

    count
}

// The first word of `s` (up to the first space, or all of `s`), as a slice of `s`.
// No copying: return a &str that points into `s`. Use as_bytes and a while loop.
fn first_word(s: &str) -> &str {
    let mut start_pos: usize = 0;

    let len: usize = s.len();
    let bytes: &[u8] = s.as_bytes();

    while start_pos < len && bytes[start_pos] == b' ' {
        start_pos += 1;
    }

    let mut end_pos: usize = 0;
    while end_pos < len && bytes[end_pos] != b' ' {
        end_pos += 1;
    }

    let word: &str = &s[start_pos..end_pos];
    word
}

// A new String with the characters of `s` in reverse order.
fn reverse(s: &str) -> String {
    let mut result: String = String::from("");

    for str_chr in s.chars() {
        result.insert(0, str_chr);
    }

    result
}

// True if `s` reads the same backwards. Reuse reverse().
fn is_palindrome(s: &str) -> bool {
    s == reverse(s)
}

// A new String made of `s` repeated n times.
fn repeat_str(s: &str, n: usize) -> String {
    let mut result: String = String::from("");

    for _ in 0..n {
        result.push_str(s);
    }

    result
}

// ---------- Part 2: mutable borrow (&mut String) ----------

// Append "[tag]" to the caller's string.
fn add_tag(s: &mut String, tag: &str) {
    s.push('[');
    s.push_str(tag);
    s.push(']');
}

// Replace every occurrence of `c` with '*' in the caller's string.
// Build a new String, then assign it back with *s = ...
fn censor(s: &mut String, c: char) {
    let mut temp: String = String::from("");

    for str_chr in s.chars() {
        if str_chr == c {
            temp.push('*');
        } else {
            temp.push(str_chr);
        }
    }

    *s = temp;
}

// ---------- Part 3: ownership in, ownership out (String) ----------

// Take a string, make it uppercase, and give it back.
fn shout(s: String) -> String {
    let mut result: String = s;
    result.make_ascii_uppercase();
    result
}

// Take two strings and return the longer one (the first on a tie). The other is dropped.
fn longer_of(a: String, b: String) -> String {
    let first_len: usize = a.len();
    let second_len: usize = b.len();

    if first_len >= second_len {
        return a;
    }

    b
}

// ---------- Part 4: Notebook (a struct that owns its strings) ----------

// Build a Note from two string slices. The Note must own its text.
fn make_note(title: &str, body: &str) -> Note {
    let result: Note = Note {
        title: String::from(title),
        body: String::from(body),
    };
    result
}

// Number of words in the body, counted by hand with chars().
fn word_count(note: &Note) -> usize {
    let mut count: usize = 0;
    let mut is_word: u8 = 0;

    for str_chr in note.body.chars() {
        if str_chr != ' ' && is_word == 0 {
            is_word = 1;
            count += 1;
        } else if str_chr == ' ' && is_word == 1 {
            is_word = 0;
        }
    }

    count
}

// Add `text` to the end of the body, separated by one space
// (no leading space if the body is empty).
fn append(note: &mut Note, text: &str) {
    if !note.body.is_empty() {
        note.body.push(' ');
    }
    note.body.push_str(text);
}

// Use the note up and return "[ARCHIVED] <title>: <body>".
fn archive(note: Note) -> String {
    let mut archive: String = String::from("[ARCHIVED] ");
    archive.push_str(&note.title);
    archive.push_str(": ");
    archive.push_str(&note.body);
    archive
}

// Combine two notes. The first is consumed and reused, the second is only read.
// Title becomes "<a>+<b>", body becomes "<a body> <b body>".
fn merge(a: Note, b: &Note) -> Note {
    let mut merged_note: Note = a;

    merged_note.title.push('+');
    merged_note.title.push_str(&b.title);
    append(&mut merged_note, &b.body);

    merged_note
}

fn main() {
    // Part 1
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    // Part 2
    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    // Part 3
    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    // Part 4
    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2));
}
