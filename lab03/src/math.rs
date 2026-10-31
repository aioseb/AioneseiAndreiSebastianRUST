// Exercise 2
pub fn panic_add(a: u32, b: u32) -> u32 {
    let a_u64: u64 = a as u64;
    let b_u64: u64 = b as u64;
    let result: u64 = a_u64 + b_u64;

    if result > u32::MAX as u64 {
        panic!("Result of a={a} + b={b} overflowed (u32)");
    }

    result as u32
}

pub fn panic_mul(a: u32, b: u32) -> u32 {
    let a_u64: u64 = a as u64;
    let b_u64: u64 = b as u64;
    let result: u64 = a_u64 * b_u64;

    if result > u32::MAX as u64 {
        panic!("Result of a={a} * b={b} overflowed (u32)");
    }

    result as u32
}

// Exercise 3
pub enum MathError {
    OverflowedResult,
}

pub fn result_add(a: u32, b: u32) -> Result<u32, MathError> {
    let a_u64: u64 = a as u64;
    let b_u64: u64 = b as u64;
    let result: u64 = a_u64 + b_u64;

    if result > u32::MAX as u64 {
        return Err(MathError::OverflowedResult);
    }
    Ok(result as u32)
}

pub fn result_mul(a: u32, b: u32) -> Result<u32, MathError> {
    let a_u64: u64 = a as u64;
    let b_u64: u64 = b as u64;
    let result: u64 = a_u64 * b_u64;

    if result > u32::MAX as u64 {
        return Err(MathError::OverflowedResult);
    }
    Ok(result as u32)
}
