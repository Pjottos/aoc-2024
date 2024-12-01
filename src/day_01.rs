use std::{arch::aarch64::*, fmt::Debug};

fn parse_nums(input: &str, mut right_value_writer: impl FnMut(&mut u32, u32)) -> (Vec<u32>, usize) {
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
            right_value_writer(&mut nums[line_count + l], vget_lane_u32::<1>(values));
        }
    }

    (nums, line_count)
}

pub fn part_1(input: &str) -> impl Debug {
    let (mut nums, line_count) = parse_nums(input, |t, v| *t = v);

    (&mut nums[..line_count]).sort_unstable();
    (&mut nums[line_count..]).sort_unstable();

    (&nums[..line_count])
        .iter()
        .zip(&nums[line_count..])
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
}

pub fn part_2(input: &str) -> impl Debug {
    let mut right_counts = vec![0u16; 100000];
    let (nums, line_count) = parse_nums(input, |_, v| right_counts[v as usize] += 1);

    let left = &nums[..line_count];

    let mut similarity_score = 0;
    for &l in left {
        similarity_score += l * u32::from(right_counts[l as usize]);
    }

    similarity_score
}
