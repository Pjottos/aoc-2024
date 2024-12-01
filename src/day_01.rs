use std::fmt::Debug;

pub fn part_1(input: &str) -> impl Debug {
    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            l.split_once("   ")
                .and_then(|(l, r)| Some((l.parse::<u32>().ok()?, r.parse::<u32>().ok()?)))
                .unwrap()
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
}

pub fn part_2(input: &str) -> impl Debug {
    let (mut left, mut right) = input
        .lines()
        .map(|l| {
            l.split_once("   ")
                .and_then(|(l, r)| Some((l.parse::<u32>().ok()?, r.parse::<u32>().ok()?)))
                .unwrap()
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    left.sort();
    right.sort();

    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut similarity_score = 0;
    while left_idx < left.len() {
        let right_slice = &right[right_idx..];
        let l = left[left_idx];
        match right_slice.binary_search(&l) {
            Ok(idx) => {
                let prev_duplicates = right_slice
                    .iter()
                    .rev()
                    .skip(right_slice.len() - idx)
                    .position(|&r| r != l)
                    .unwrap_or(idx);
                let remaining = &right_slice[idx + 1..];
                let next_duplicates = remaining
                    .iter()
                    .position(|&r| r != l)
                    .unwrap_or(remaining.len());
                let duplicates = prev_duplicates + 1 + next_duplicates;
                similarity_score += l as usize * duplicates;
            }
            Err(idx) => {
                right_idx += idx;
            }
        }
        left_idx += 1;
    }

    similarity_score
}
