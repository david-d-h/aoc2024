pub mod part1;
pub mod part2;

pub const INPUT: &[u8] = include_bytes!("../../res/day3.txt");
pub const SAMPLE: &[u8] =
    b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

pub const KEYWORD_MUL: [u8; 3] = *b"mul";
pub const KEYWORD_DO: [u8; 2] = *b"do";

pub const fn allowed_closing(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9' | b')')
}

pub const fn allowed_comma(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9' | b',')
}

/// Converts a single byte (b'0'..=b'9') to a u32 integer.
pub const fn byte_to_int(byte: u8) -> u32 {
    (byte - b'0') as u32
}

/// Construct a number from a slice of bytes.
pub const fn construct_number(bytes: &[u8]) -> u32 {
    let mut accum = 0;
    let mut idx = 0;
    loop {
        if idx == bytes.len() {
            break accum;
        }

        accum = accum * 10 + byte_to_int(bytes[idx]);
        idx += 1;
    }
}

pub const fn left_<T>(left: T, _: ()) -> T {
    left
}

pub fn skip_until<const N: usize>(input: &[u8], cursor: &mut usize, bytes: [u8; N]) -> bool {
    while *cursor < input.len()
        && let byte = input[*cursor]
        && byte != bytes[0]
    {
        *cursor += 1;
    }

    left_(
        input.len() - *cursor >= N && input[*cursor..*cursor + N] == bytes,
        *cursor += bytes.len(),
    )
}

pub const fn exact(input: &[u8], cursor: &mut usize, byte: u8) -> bool {
    left_(
        input.len() - *cursor >= 1 && input[*cursor] == byte,
        *cursor += 1,
    )
}

pub fn seek_for(input: &[u8], cursor: &mut usize, seek: u8, allowed: fn(u8) -> bool) -> bool {
    while *cursor < input.len()
        && let byte = input[*cursor]
        && allowed(byte)
    {
        *cursor += 1;
        if byte == seek {
            return true;
        }
    }
    false
}

pub fn parsing_eval(input: &[u8]) -> u32 {
    let mut cursor = 0usize;
    let mut accum = 0u32;

    loop {
        if cursor > input.len() {
            break accum;
        }

        if !skip_until(input, &mut cursor, KEYWORD_MUL) || !exact(input, &mut cursor, b'(') {
            continue;
        }

        let at_opening = cursor;

        if !seek_for(input, &mut cursor, b',', allowed_comma) {
            continue;
        }

        let at_comma = cursor;

        if !seek_for(input, &mut cursor, b')', allowed_closing) {
            continue;
        }

        let at_closing = cursor;

        accum += construct_number(&input[at_opening..at_comma - 1])
            * construct_number(&input[at_comma..at_closing - 1])
    }
}
