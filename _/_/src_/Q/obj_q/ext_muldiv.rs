fn mul_div(a: u128, b: u128, denominator: u128) -> u128 {
    assert!(denominator != 0, "division by zero");

    // 256-bit multiply: (a * b) = high * 2^128 + low
    let (prod_low, carry) = a.overflowing_mul(b);
    let a_hi = a >> 64;
    let a_lo = a & 0xFFFFFFFFFFFFFFFF;
    let b_hi = b >> 64;
    let b_lo = b & 0xFFFFFFFFFFFFFFFF;

    // cross terms (safe from overflow)
    let mid = a_lo * b_hi + a_hi * b_lo;

    // reconstruct 256-bit product as (high, low)
    let mut prod1 = (a_hi * b_hi) + (mid >> 64);
    let mut prod0 = prod_low;

    // Handle short circuit if high part is zero
    if prod1 == 0 {
        return prod0 / denominator;
    }

    // Ensure result < 2^128
    assert!(prod1 < denominator, "overflow");

    // Compute remainder using mulmod (a * b % denominator)
    let remainder = (a as u128 * b as u128) % denominator;

    // Subtract remainder from (prod1, prod0)
    if remainder > prod0 {
        prod1 -= 1;
        prod0 = prod0.wrapping_sub(remainder);
    } else {
        prod0 -= remainder;
    }

    // Factor out powers of two from denominator
    let twos = denominator & (!denominator + 1); // lowest bit set
    let denominator_div = denominator / twos;
    prod0 /= twos;

    // Compute 2^128 / twos (simulate flipping)
    let mut twos_flipped = (1u128 << 128) / twos;

    // Shift bits from high into low
    prod0 = prod0 + prod1 * twos_flipped;

    // Modular inverse via Newton-Raphson
    let mut inv = 3 * denominator_div ^ 2;
    inv *= 2 - denominator_div * inv;
    inv *= 2 - denominator_div * inv;
    inv *= 2 - denominator_div * inv;
    inv *= 2 - denominator_div * inv;
    inv *= 2 - denominator_div * inv;
    inv *= 2 - denominator_div * inv;

    // Final result
    let result = prod0 * inv;
    result
}

fn mul_div_i128(a: i128, b: i128, denominator: i128) -> i128 {
    assert!(denominator != 0, "division by zero");

    // Determine the sign of the result
    let sign = ((a < 0) as i8 ^ (b < 0) as i8 ^ (denominator < 0) as i8) == 1;

    // Work with absolute values to avoid signed overflow
    let a_abs = a.abs() as u128;
    let b_abs = b.abs() as u128;
    let d_abs = denominator.abs() as u128;

    // Full 256-bit multiplication: a_abs * b_abs
    let prod = (a_abs as u128) * (b_abs as u128);

    // Perform division
    let result_unsigned = prod / d_abs;

    // Convert back to signed, applying the sign
    if sign {
        // Make sure we don’t overflow on negation
        assert!(result_unsigned <= i128::MAX as u128 + 1);
        -(result_unsigned as i128)
    } else {
        assert!(result_unsigned <= i128::MAX as u128);
        result_unsigned as i128
    }
}
