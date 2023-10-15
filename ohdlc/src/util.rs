/// Mask of the value bits of a continuation byte.
const CONT_MASK: u8 = 0b0011_1111;

// copied from `core::str::validations::next_code_point` and modified for use without iterator
// TODO: this was unsafe, should we think about that?
pub fn next_utf8_point(bytes: &[u8], mut start: usize) -> (char, usize) {
    // Decode UTF-8
    let x = bytes[start];
    if x < 128 {
        unsafe {
            start += 1;
            return (char::from_u32_unchecked(x as u32), start);
        }
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]
    // NOTE: Performance is sensitive to the exact formulation here
    let init = (x & (0x7F >> 2)) as u32;
    // SAFETY: `bytes` produces an UTF-8-like string,
    // so the iterator must produce a value here.
    start += 1;
    let y = unsafe { *bytes.get_unchecked(start) };
    let mut ch = (init << 6) | (y & CONT_MASK) as u32;
    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        // SAFETY: `bytes` produces an UTF-8-like string,
        // so the iterator must produce a value here.
        start += 1;
        let z = unsafe { *bytes.get_unchecked(start) };
        let y_z = (((y & CONT_MASK) as u32) << 6) | (z & CONT_MASK) as u32;
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            // SAFETY: `bytes` produces an UTF-8-like string,
            // so the iterator must produce a value here.
            start += 1;
            let w = unsafe { *bytes.get_unchecked(start) };
            ch = (init & 7) << 18 | (y_z << 6) | (w & CONT_MASK) as u32;
        }
    }

    start += 1;
    unsafe { (char::from_u32_unchecked(ch), start) }
}
