pub fn sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0
    }
    let mut x_0 = n / 2;
    let mut x_1 = (x_0 + n / x_0) / 2;
    while x_1 < x_0 {
        x_0 = x_1;
        x_1 = (x_0 + n / x_0) / 2;
    }
    x_0
}

pub fn fold_128_bit_product_mod(a: u128, b: u128, z: u128) -> u128 {
    (((((b % z) << 64) | (a >> 64)) % z) << 64) | (a & 0xFFFFFFFFFFFFFFFF)
}

pub fn wide_mul(x: u128, y: u128) -> (u128, u128) {
    let x_hi: u128 = x >> 64;
    let x_lo: u128 = x & 0xFFFFFFFFFFFFFFFF;
    let y_hi: u128 = y >> 64;
    let y_lo: u128 = y & 0xFFFFFFFFFFFFFFFF;
    let lo_lo: u128 = x_lo * y_lo;
    let lo_hi: u128 = x_lo * y_hi;
    let hi_lo: u128 = x_hi * y_lo;
    let hi_hi: u128 = x_hi * y_hi;
    let m: u128 = lo_hi + hi_lo;
    let c: u128 = ((m < lo_hi) as u128) << 64;
    let m_lo: u128 = m << 64;
    let m_hi: u128 = m >> 64;
    let a: u128 = lo_lo.wrapping_add(m_lo);
    let b: u128 = hi_hi + m_hi + c + ((a < lo_lo) as u128);
    (a, b)
}