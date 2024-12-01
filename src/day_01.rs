use std::{arch::aarch64::*, fmt::Debug};

fn parse_nums(input: &str) -> (Vec<u32>, usize) {
    let input = input.as_bytes();
    let line_len = 5 + 3 + 5 + 1;
    let line_count = input.len() / line_len;
    assert!(line_count < u16::MAX as usize);
    let mut nums = vec![0; line_count * 2];
    unsafe {
        for (l, line) in input.chunks_exact(line_len).enumerate() {
            // We may read out of bounds, but the way we allocate input makes this sound
            let line = vld1q_u8(line.as_ptr());
            let digits = vsubq_u8(line, vdupq_n_u8(b'0'));
            let low_a = vreinterpret_u32_u8(vqtbl1_u8(digits, vcreate_u8(0xFF0AFF0C_FF02FF04)));
            let low_b = vreinterpret_u32_u8(vqtbl1_u8(digits, vcreate_u8(0xFF09FF0B_FF01FF03)));
            let high = vreinterpret_u32_u8(vqtbl1_u8(digits, vcreate_u8(0xFFFFFF08_FFFFFF00)));
            let low_split = vmla_u32(low_a, low_b, vdup_n_u32(10));
            let low = vshr_n_u32::<16>(vmul_u32(low_split, vdup_n_u32(0x1_0000 + 100)));
            let values = vadd_u32(low, vmul_u32(high, vdup_n_u32(10_000)));
            nums[l] = vget_lane_u32::<0>(values);
            nums[line_count + l] = vget_lane_u32::<1>(values);
        }
    }

    (nums, line_count)
}

pub fn part_1(input: &str) -> impl Debug {
    let (mut nums, line_count) = parse_nums(input);

    (&mut nums[..line_count]).sort();
    (&mut nums[line_count..]).sort();

    (&nums[..line_count])
        .iter()
        .zip(&nums[line_count..])
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
}

pub fn part_2(input: &str) -> impl Debug {
    let (mut nums, line_count) = parse_nums(input);

    (&mut nums[..line_count]).sort();
    (&mut nums[line_count..]).sort();

    let left = &nums[..line_count];
    let right = &nums[line_count..];

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
