use super::{
    allowed_closing, allowed_comma, construct_number, exact, parsing_eval, seek_for, skip_until,
    INPUT, KEYWORD_MUL,
};

pub type Instruction = (u32, u32);

// slower implementation because of alloc, look in mod.rs
pub fn parse(input: &[u8]) -> Vec<Instruction> {
    let mut cursor = 0usize;
    let mut instructions = Vec::new();

    loop {
        if cursor > input.len() {
            break instructions;
        }

        if !skip_until(input, &mut cursor, KEYWORD_MUL) || !exact(input, &mut cursor, b'(') {
            continue;
        }

        let before_comma = cursor;

        if !seek_for(input, &mut cursor, b',', allowed_comma) {
            continue;
        }

        let at_comma = cursor;

        if !seek_for(input, &mut cursor, b')', allowed_closing) {
            continue;
        }

        let at_closing = cursor;

        let instruction = (
            construct_number(&input[before_comma..at_comma - 1]),
            construct_number(&input[at_comma..at_closing - 1]),
        );

        instructions.push(instruction);
    }
}

pub fn solution(instructions: Vec<Instruction>) -> u32 {
    instructions
        .into_iter()
        .fold(0, |acc, (lhs, rhs)| acc + lhs * rhs)
}

pub fn run() -> String {
    // uses the fast (and reused in part 2) solution located in mod.rs
    let result = parsing_eval(INPUT);

    format!("result of multiplication instructions: {result}")
}
