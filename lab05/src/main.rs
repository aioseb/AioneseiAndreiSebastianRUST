use serde_derive::Deserialize;
use std::{fs, io};

struct Canvas {
    grid: [[char; 55]; 100],
}

struct Game {
    grid: [[char; 20]; 20],
}

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}

#[derive(Debug, Deserialize)]
struct Data {
    students: [Student; 4],
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn new_canvas() -> Canvas {
    Canvas {
        grid: [[' '; 55]; 100],
    }
}

// Read game board from given path. If empty, return an empty board
fn new_game(path: &str) -> Result<Game, io::Error> {
    // If path is empty, initialize an empty board
    let mut game: Game = Game {
        grid: [[' '; 20]; 20],
    };

    if path.is_empty() {
        return Ok(game);
    }

    let content: String = read_file(path)?;
    let lines = content.lines();

    for (row, line) in lines.enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == 'x' {
                game.grid[row][col] = 'x';
            }
        }
    }
    
    Ok(game)
}

fn set_pixels(canvas: &mut Canvas, pixels: &[(u8, u8, u8)]) {
    for pixel in pixels {
        let x: usize = pixel.0 as usize;
        let y: usize = pixel.1 as usize;
        let ch: char = pixel.2 as char;
        canvas.grid[x][y] = ch;
    }
}

fn print(canvas: Canvas) {
    for row in canvas.grid {
        for ch in row {
            print!("{ch}");
        }
        println!();
    }
}

// Transition into the next state in the Conway's game of life
fn transition(game: Game) -> Result<Game, io::Error> {
    let rows: usize = game.grid.len();
    let cols: usize = game.grid[0].len();
    let mut trans: Game = new_game("")?;

    let di: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dj: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 0..rows {
        for j in 0..cols {
            // Count the number of neighbours for every cell
            let mut nei = 0;

            for k in 0..8 {
                // Check for out of bounds coordinates
                if i == 0 && di[k] == -1 {
                    continue;
                }
                if i == rows - 1 && di[k] == 1 {
                    continue;
                }
                if j == 0 && dj[k] == -1 {
                    continue;
                }
                if j == cols - 1 && dj[k] == 1 {
                    continue;
                }

                let vi = (i as i32 + di[k]) as usize;
                let vj = (j as i32 + dj[k]) as usize;

                if game.grid[vi][vj] == 'x' {
                    nei += 1
                };
            }

            // Check survival
            if (nei == 2 || nei == 3) && game.grid[i][j] == 'x' {
                trans.grid[i][j] = 'x';
            }

            // Check 'revival'
            if nei == 3 && game.grid[i][j] == ' ' {
                trans.grid[i][j] = 'x';
            }
        }
    }

    // Count the number of neighbours for every cell
    Ok(trans)
}


fn print_game(game: &Game) {
    for row in &game.grid {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
}

fn problem_one() -> Result<(), io::Error> {
    let content = read_file("./text_files/students.txt")?;
    let lines = content.lines();

    let mut min_name = "";
    let mut max_name = "";

    let mut min_age = 200;
    let mut max_age = 0;

    for line in lines {
        let mut row = line.split(",");

        let name = row.next().unwrap_or("0");
        let _phone = row.next().unwrap_or("0");
        let age = row.next().unwrap_or("0");

        let age_int: u8 = age.parse().unwrap_or(0);

        if min_age > age_int {
            min_age = age_int;
            min_name = name;
        }

        if max_age < age_int {
            max_age = age_int;
            max_name = name;
        }
    }

    println!("Oldest: {max_name}");
    println!("Youngest: {min_name}");

    Ok(())
}

fn problem_two() {
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(
        c,
        &[
            (2, 16, 95),
            (5, 35, 92),
            (6, 3, 95),
            (2, 11, 95),
            (5, 3, 95),
        ],
    );
    set_pixels(
        c,
        &[
            (2, 38, 95),
            (4, 9, 40),
            (3, 41, 124),
            (2, 37, 95),
            (2, 25, 124),
        ],
    );
    set_pixels(
        c,
        &[
            (5, 27, 124),
            (2, 27, 124),
            (4, 0, 124),
            (3, 35, 47),
            (2, 18, 95),
        ],
    );
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(
        c,
        &[
            (6, 30, 95),
            (5, 45, 95),
            (4, 31, 124),
            (4, 7, 124),
            (3, 43, 39),
        ],
    );
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(
        c,
        &[
            (3, 44, 95),
            (3, 19, 92),
            (5, 23, 95),
            (3, 8, 47),
            (2, 10, 95),
        ],
    );
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 10, 95),
            (4, 44, 95),
            (2, 9, 95),
            (0, 32, 95),
            (5, 2, 95),
        ],
    );
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(
        c,
        &[
            (3, 46, 92),
            (5, 25, 44),
            (1, 43, 124),
            (5, 46, 47),
            (3, 15, 47),
        ],
    );
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(
        c,
        &[
            (5, 44, 95),
            (2, 2, 95),
            (5, 10, 95),
            (5, 9, 95),
            (4, 43, 124),
        ],
    );
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(
        c,
        &[
            (4, 40, 124),
            (3, 3, 95),
            (4, 4, 124),
            (6, 31, 47),
            (3, 4, 96),
        ],
    );
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 12, 92),
            (2, 32, 95),
            (5, 37, 95),
            (5, 26, 95),
            (5, 39, 47),
        ],
    );
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(
        c,
        &[
            (5, 36, 95),
            (7, 30, 95),
            (6, 4, 47),
            (4, 24, 95),
            (1, 32, 95),
        ],
    );
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);
}

fn problem_three() -> Result<(), io::Error> {
    let content = read_file("./text_files/students.json")?;
    println!("{content}");
    let data: Data = serde_json::from_str(&content)?;
    let students: [Student; 4] = data.students;

    let mut min_name: &str = "";
    let mut max_name: &str = "";

    let mut min_age = 200;
    let mut max_age = 0;

    for student in &students {
        let name: &String = &student.name;
        let age_int = student.age;
        let _phone: &String = &student.phone;

        if min_age > age_int {
            min_age = age_int;
            min_name = name.as_str();
        }

        if max_age < age_int {
            max_age = age_int;
            max_name = name.as_str();
        }
    }

    println!("Oldest: {max_name}");
    println!("Youngest: {min_name}");

    Ok(())
}

fn bonus(path: &str, iterations: u8) -> Result<(), io::Error> {
    let mut game: Game = new_game(path)?;

    // Print game board in the given iterations
    println!("Beginning of Conway's Game of Life with {iterations} iterations:");

    for i in 0..=iterations {
        println!("Iteration {i}:");
        print_game(&game);
        game = transition(game)?;
    }

    Ok(())
}

fn main() {
    match problem_one() {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }

    problem_two();

    match problem_three() {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }

    match bonus("./text_files/life.game", 5) {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }

    match bonus("./text_files/life2.game", 5) {
        Ok(()) => {}
        Err(error) => println!("{error}"),
    }
}
