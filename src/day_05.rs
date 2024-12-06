use std::arch::aarch64::*;
use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug};

const RULE_COUNT: usize = 1176;
const RULES_END: usize = RULE_COUNT * 6;

pub fn part_1(input: &str) -> impl Debug {
    let input = input.as_bytes();
    assert_eq!(RULES_END % (16 * 3), 0);

    unsafe {
        let mut result = 0;
        let rule_map = parse_rules(input);
        parse_updates(input, |update_nums| {
            let is_invalid = update_nums.iter().enumerate().any(|(i, &n)| {
                let high = (n >= 64) as usize;
                let bit = 1 << (n % 64);
                update_nums[i + 1..]
                    .iter()
                    .any(|n2| rule_map[(n2 * 2) as usize + high] & bit != 0)
            });
            if !is_invalid {
                result += u32::from(update_nums[update_nums.len() / 2]);
            }
        });

        result
    }
}
pub fn part_2(input: &str) -> impl Debug {
    let input = input.as_bytes();
    assert_eq!(RULES_END % (16 * 3), 0);

    unsafe {
        let mut result = 0;
        let rule_map = parse_rules(input);
        parse_updates(input, |update_nums| {
            let is_invalid = update_nums.iter().enumerate().any(|(i, &n)| {
                let high = (n >= 64) as usize;
                let bit = 1 << (n % 64);
                update_nums[i + 1..]
                    .iter()
                    .any(|&n2| rule_map[(n2 * 2) as usize + high] & bit != 0)
            });
            if is_invalid {
                let idx = update_nums.len() / 2;
                result += u32::from(
                    *update_nums
                        .select_nth_unstable_by(idx, |&a, &b| {
                            match (
                                rule_map[(a * 2) as usize + b as usize / 64] & (1 << (b % 64)) != 0,
                                rule_map[(b * 2) as usize + a as usize / 64] & (1 << (a % 64)) != 0,
                            ) {
                                (false, false) => Ordering::Equal,
                                (false, true) => Ordering::Greater,
                                (true, false) => Ordering::Less,
                                (true, true) => panic!("unsolvable rules"),
                            }
                        })
                        .1,
                );
            }
        });

        result
    }
}

unsafe fn parse_rules(input: &[u8]) -> [u64; 128 * 2] {
    let mut rule_map = [0u64; 128 * 2];

    let mut last_key = 0;
    let mut values_low = 0;
    let mut values_high = 0;
    for i in (0..RULES_END).step_by(16 * 3) {
        let uint8x16x3_t(ten, one, _) = vld3q_u8(input.as_ptr().add(i));
        let ten = vsubq_u8(ten, vdupq_n_u8(b'0'));
        let one = vsubq_u8(one, vdupq_n_u8(b'0'));
        let nums = vaddq_u8(one, vaddq_u8(vshlq_n_u8::<3>(ten), vshlq_n_u8::<1>(ten)));

        let keys = vmovn_u16(vreinterpretq_u16_u8(nums));
        let keys = vget_lane_u64::<0>(vreinterpret_u64_u8(keys));
        let values = vshrn_n_u16::<8>(vreinterpretq_u16_u8(nums));
        let values = vget_lane_u64::<0>(vreinterpret_u64_u8(values));

        for j in 0..8 {
            let shift = j * 8;
            let key = (keys >> shift) as usize & 0x7F;
            if key != last_key {
                rule_map[last_key * 2] |= values_low;
                rule_map[last_key * 2 + 1] |= values_high;
                values_low = 0;
                values_high = 0;
            }
            last_key = key;
            let value = (values >> shift) & 0x7F;
            if value >= 64 {
                values_high |= 1 << (value % 64);
            } else {
                values_low |= 1 << (value % 64);
            }
        }
    }
    // Get the very last values in
    rule_map[last_key * 2] |= values_low;
    rule_map[last_key * 2 + 1] |= values_high;

    rule_map
}

unsafe fn parse_updates(input: &[u8], mut handler: impl FnMut(&mut [u8])) {
    let mut update_buf: Vec<u8> = vec![];

    for i in (RULES_END + 1..input.len()).step_by(16 * 3) {
        let uint8x16x3_t(ten, one, sep) = vld3q_u8(input.as_ptr().add(i));
        let ten = vandq_u8(ten, vdupq_n_u8(0xF));
        let one = vandq_u8(one, vdupq_n_u8(0xF));
        let nums = vaddq_u8(one, vaddq_u8(vshlq_n_u8::<3>(ten), vshlq_n_u8::<1>(ten)));

        let partial_count = update_buf.len();
        update_buf.reserve(16);
        vst1q_u8(update_buf.as_mut_ptr().add(update_buf.len()), nums);
        update_buf.set_len(update_buf.len() + 16);

        let newlines = vceqq_u8(sep, vdupq_n_u8(b'\n'));
        let mut ends_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(
            vreinterpretq_u16_u8(newlines),
        )));

        let mut buf_idx = 0;
        while ends_bits != 0 {
            let zeros = ends_bits.trailing_zeros();
            let offset = 1 + zeros / 4;
            ends_bits &= !(0xF << zeros);

            let end = partial_count + offset as usize;
            let update_nums = &mut update_buf[buf_idx..end];
            handler(update_nums);

            buf_idx = end;
        }
        update_buf.drain(0..buf_idx);
    }
}
