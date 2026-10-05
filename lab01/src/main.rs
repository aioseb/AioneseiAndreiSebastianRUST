// Exercitiul 1
fn is_prime(n: i32) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 || n < 2 {
        return false;
    }

    let mut i: i32 = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

// Exercitiul 2
fn is_coprime(a: i32, b: i32) -> bool {
    // Cauta GCD-ul numerelor
    let mut remainder: i32;
    let mut first_val: i32 = a;
    let mut second_val: i32 = b;

    while second_val != 0 {
        remainder = first_val % second_val;
        first_val = second_val;
        second_val = remainder;
    }

    // Daca GCD-ul este 1, atunci a si b sunt coprime
    if first_val == 1 {
        return true;
    }
    false
}

// Exercitiul 3
fn nth_bottle_of_beer(n: i32) {
    let mut plural: &str;

    // Vedem daca este plural
    if n > 1 {
        plural = "s";
    } else {
        plural = "";
    }

    println!("{} bottle{} of beer on the wall,", n, plural);
    println!("{} bottle{} of beer.", n, plural);
    println!("Take one down, pass it around,");

    if n == 2 {
        plural = "";
    }

    if n > 1 {
        println!("{} bottle{} of beer on the wall.", n - 1, plural);
    } else {
        println!("No bottles of beer on the wall");
    }
}

fn main() {
    // Afisam numerele prime de la 0 la 100
    print!("Numerele prime de la 0 la 100: ");
    for n in 0..100 {
        if is_prime(n) {
            print!("{n} ");
        }
    }
    println!("\n");

    // Afisam perechile de numere prime de la 0 la 100
    print!("Perechile de numere coprime: ");
    for a in 0..99 {
        for b in (a + 1)..100 {
            if is_coprime(a, b) {
                print!("({}, {}) ", a, b);
            }
        }
    }
    println!("\n");

    // Cantam "99 bottles of beer"
    for bottle in (1..99).rev() {
        nth_bottle_of_beer(bottle);
        println!();
    }
}
