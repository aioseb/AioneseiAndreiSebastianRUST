// Exercise 1
pub fn next_prime(x: u16) -> Option<u16> {
    // Treat base cases
    if x == 0 || x == 1 {
        return Some(2);
    }

    if x == 2 {
        return Some(3);
    }

    let mut aux: u32 = (x as u32) + 1;
    if aux.is_multiple_of(2) {
        aux += 1;
    }

    // Search for the next prime number, starting from x + 1 ( +1 if x is odd)
    // If it goes above u16, then return None
    while aux < u16::MAX as u32 {
        let mut div: u32 = 3;

        while div * div <= aux {
            if aux.is_multiple_of(div) {
                break;
            }

            div += 2;
        }

        // Next prime found
        if div * div > aux {
            return Some(aux as u16);
        }

        aux += 2;
    }

    None
}
