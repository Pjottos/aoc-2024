use std::fmt::Debug;

pub fn part_1(input: &str) -> impl Debug {
    let input = input.as_bytes();
    let line_len = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let line_count = input.len() / line_len;
    assert_eq!(line_len - 1, line_count);
    let mut matches = 0;

    for i in input
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| (b == b'X').then_some(i))
    {
        fn check(input: &[u8], mut idx: impl FnMut(usize) -> usize) -> u32 {
            (1..4).all(|j| input.get(idx(j)) == b"XMAS".get(j)) as u32
        }
        // right
        matches += check(input, |j| i + j);
        // left
        matches += check(input, |j| i.wrapping_sub(j));
        // down
        matches += check(input, |j| i + j * line_len);
        // up
        matches += check(input, |j| i.wrapping_sub(j * line_len));

        let x = i % line_len;
        let y = i / line_len;
        // down-right
        if x < line_len - 4 && y <= line_count - 4 {
            matches += check(input, |j| i + j * line_len + j);
        }
        // down-left
        if x >= 3 && y <= line_count - 4 {
            matches += check(input, |j| (i + j * line_len).wrapping_sub(j));
        }
        // up-right
        if x < line_len - 4 && y >= 3 {
            matches += check(input, |j| i.wrapping_sub(j * line_len) + j);
        }
        // up-left
        if x >= 3 && y >= 3 {
            matches += check(input, |j| i.wrapping_sub(j * line_len).wrapping_sub(j));
        }
    }

    matches
}

pub fn part_2(input: &str) -> impl Debug {
    let input = input.as_bytes();
    let mut matches = 0;
    let line_len = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let line_count = input.len() / line_len;
    assert_eq!(line_len - 1, line_count);

    for i in input
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| (b == b'A').then_some(i))
    {
        let x = i % line_len;
        let y = i / line_len;
        if x == 0 || x >= line_len - 2 || y == 0 || y >= line_count - 1 {
            continue;
        }
        let a = input[i + line_len + 1];
        let b = input[i - line_len - 1];
        let up_left = (a == b'M' && b == b'S') || (a == b'S' && b == b'M');
        let a = input[i + line_len - 1];
        let b = input[i - line_len + 1];
        let down_right = (a == b'M' && b == b'S') || (a == b'S' && b == b'M');
        matches += (up_left && down_right) as u32;
    }

    matches
}
