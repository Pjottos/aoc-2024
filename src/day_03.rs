use std::fmt::Debug;

pub fn part_1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut sum = 0;

    let mut i = 0;
    while i < input.len() {
        if input.get(i..i + 4) == Some(b"mul(") {
            i += 4;
            let Some(a) = parse_num(input, &mut i, b',') else {
                continue;
            };
            let Some(b) = parse_num(input, &mut i, b')') else {
                continue;
            };
            sum += a * b;
        } else {
            i += 1;
        }
    }
    sum
}

pub fn part_2(input: &str) -> impl Debug {
    input
        .split("do()")
        .flat_map(|s| s.splitn(2, "don't()").next())
        .map(part_1)
        .sum::<u32>()
}

fn parse_num(input: &[u8], i: &mut usize, terminator: u8) -> Option<u32> {
    let mut num = 0;
    for j in 0..4 {
        // Return None when out of bounds because there always needs to be a character
        // after the number
        let character = *input.get(*i)?;
        if character == terminator {
            let res = (j != 0).then_some(num);
            *i += 1;
            return res;
        } else if j == 3 {
            return None;
        }
        let digit = u32::from(character.wrapping_sub(b'0'));
        if digit > 9 {
            return None;
        }
        num = digit + num * 10;
        *i += 1;
    }
    None
}
