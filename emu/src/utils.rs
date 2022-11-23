use std::mem::transmute;

/// most significant bit
#[allow(dead_code)]
pub fn msb(n: u32) -> u32 {
    // Below steps set bits after
    // MSB (including MSB)
    let mut n = n;
    // Suppose n is 273 (binary
    // is 100010001). It does following
    // 100010001 | 010001000 = 110011001
    n |= n >> 1;

    // This makes sure 4 bits
    // (From MSB and including MSB)
    // are set. It does following
    // 110011001 | 001100110 = 111111111
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;

    // The naive approach would increment n by 1,
    // so only the MSB+1 bit will be set,
    // So now n theoretically becomes 1000000000.
    // All the would remain is a single bit right shift:
    //    n = n + 1;
    //    return (n >> 1);
    //
    // ... however, this could overflow the type.
    // To avoid overflow, we must retain the value
    // of the bit that could overflow:
    //     n & (1 << ((sizeof(n) * CHAR_BIT)-1))
    // and OR its value with the naive approach:
    //     ((n + 1) >> 1)
    n = ((n + 1) >> 1) | (n & (1 << (4 * 8) - 1));
    return n;
}

// sign extension
// https://en.wikipedia.org/wiki/Sign_extension
pub fn sext(n: u32, from: u32, to: u32) -> u32 {
    let shift = to - from;
    let n = n << shift;
    let n: i32 = unsafe { transmute(n) };
    let n = n >> shift;
    unsafe { transmute(n) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sext_pos() {
        let n = 0b01010;
        let expected: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1010;
        let s = sext(n, 4, 32);
        assert_eq!(s, expected)
    }

    #[test]
    fn msb_test() {
        let n = 0b10100;
        let s = msb(n);
        assert_eq!(s, 2_u32.pow(4));
    }
}
