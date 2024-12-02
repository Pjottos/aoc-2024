use std::fmt::Debug;

pub fn part_1(input: &str) -> impl Debug {
    input
        .lines()
        .filter(|l| {
            let nums = l
                .split(' ')
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            is_safe(&nums)
        })
        .count()
}

pub fn part_2(input: &str) -> impl Debug {
    input
        .lines()
        .filter(|l| {
            let nums = l
                .split(' ')
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if !is_safe(&nums) {
                for i in 0..nums.len() {
                    let mut tmp = nums.clone();
                    tmp.remove(i);
                    if is_safe(&tmp) {
                        return true;
                    }
                }
                return false;
            }

            true
        })
        .count()
}

fn is_safe(nums: &[i32]) -> bool {
    let mut last = None;
    let mut increasing = None;
    for &num in nums {
        if let Some(last) = last.as_ref() {
            let diff: i32 = num - last;
            if diff == 0 || diff.abs() > 3 || increasing.map_or(false, |inc| (diff > 0) != inc) {
                return false;
            }
            if increasing.is_none() {
                increasing = Some(diff > 0);
            }
        }
        last = Some(num);
    }
    true
}
