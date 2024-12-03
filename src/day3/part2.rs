use super::{exact, parsing_eval, skip_until, INPUT, KEYWORD_DO};

pub fn with_blocking(input: &[u8]) -> u32 {
    let mut cursor = 0usize;
    let mut accum = 0u32;
    let mut unblocked_at = Some(0usize);

    loop {
        if cursor > input.len() {
            break accum;
        }

        // find the next do or don't instruction, do and don't instructions are
        // never used with invalid syntax, so parsing them is simple.
        if !skip_until(input, &mut cursor, KEYWORD_DO) {
            // parse the remainder of the input
            if let Some(pos) = unblocked_at {
                accum += parsing_eval(&input[pos..cursor - 2]);
            }

            break accum;
        }
        // a do instruction is always followed by "(" or "n't()" in official AOC input.
        else if exact(input, &mut cursor, b'(') {
            cursor += 1; // increment once for parsing ")".
            if unblocked_at.is_none() {
                unblocked_at = Some(cursor);
            }
            continue;
        } else {
            // increment by four because `exact` has already incremented once
            // for (trying) parsing "(", this leaves "'t()"
            cursor += 4;
            if let Some(pos) = unblocked_at {
                accum += parsing_eval(&input[pos..cursor - 7]); // length of "don't()" = 7
                unblocked_at = None;
            }
            continue;
        }
    }
}

pub fn run() -> String {
    let result = with_blocking(INPUT);

    format!("result of (not blocked) multiplication instructions: {result}")
}
