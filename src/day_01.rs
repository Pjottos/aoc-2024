use std::{arch::aarch64::*, fmt::Debug};

fn parse_nums(input: &str, mut right_value_writer: impl FnMut(uint32x4_t)) -> Vec<u32> {
    let input = input.as_bytes();
    let line_len: u8 = 5 + 3 + 5 + 1;
    let line_count = input.len() / line_len as usize;
    assert!(line_count < u16::MAX as usize);
    assert!(line_count % 4 == 0);
    let mut left = Vec::with_capacity(line_count);
    unsafe {
        for chunk in input.chunks_exact(line_len as usize * 4) {
            // We may read out of bounds, but the way we allocate input makes this sound
            let chunk = (chunk.as_ptr() as *const uint8x16x4_t).read_unaligned();

            let idxs = ([
                4,
                2,
                4 + line_len,
                2 + line_len,
                4 + line_len * 2,
                2 + line_len * 2,
                4 + line_len * 3,
                2 + line_len * 3,
                12,
                10,
                12 + line_len,
                10 + line_len,
                12 + line_len * 2,
                10 + line_len * 2,
                12 + line_len * 3,
                10 + line_len * 3,
            ]
            .as_ptr() as *const uint8x16_t)
                .read_unaligned();
            let ac = vsubq_u8(vqtbl4q_u8(chunk, idxs), vdupq_n_u8(b'0'));

            let idxs = ([
                3,
                1,
                3 + line_len,
                1 + line_len,
                3 + line_len * 2,
                1 + line_len * 2,
                3 + line_len * 3,
                1 + line_len * 3,
                11,
                9,
                11 + line_len,
                9 + line_len,
                11 + line_len * 2,
                9 + line_len * 2,
                11 + line_len * 3,
                9 + line_len * 3,
            ]
            .as_ptr() as *const uint8x16_t)
                .read_unaligned();
            let bd = vsubq_u8(vqtbl4q_u8(chunk, idxs), vdupq_n_u8(b'0'));

            let ab_cd = vmlaq_u8(ac, bd, vdupq_n_u8(10));
            let ab_cd_l = vmovl_u8(vget_low_u8(ab_cd));
            let ab_cd_r = vmovl_high_u8(ab_cd);

            let abcd_l = vshrq_n_u32::<16>(vmulq_u32(
                vreinterpretq_u32_u16(ab_cd_l),
                vdupq_n_u32(0x1_0000 + 100),
            ));
            let abcd_r = vshrq_n_u32::<16>(vmulq_u32(
                vreinterpretq_u32_u16(ab_cd_r),
                vdupq_n_u32(0x1_0000 + 100),
            ));

            let idxs = ([
                0,
                0xFF,
                0 + line_len,
                0xFF,
                0 + line_len * 2,
                0xFF,
                0 + line_len * 3,
                0xFF,
                8,
                0xFF,
                8 + line_len,
                0xFF,
                8 + line_len * 2,
                0xFF,
                8 + line_len * 3,
                0xFF,
            ]
            .as_ptr() as *const uint8x16_t)
                .read_unaligned();
            let e = vsubq_u8(
                vqtbl4q_u8(chunk, idxs),
                vandq_u8(vdupq_n_u8(b'0'), vreinterpretq_u8_u16(vdupq_n_u16(0x00FF))),
            );
            let e = vreinterpretq_u16_u8(e);
            let e_l = vmovl_u16(vget_low_u16(e));
            let e_r = vmovl_high_u16(e);

            let abcde_l = vmlaq_u32(abcd_l, e_l, vdupq_n_u32(10_000));
            let abcde_r = vmlaq_u32(abcd_r, e_r, vdupq_n_u32(10_000));
            let mut store = [0; 4];
            vst1q_u32(store.as_mut_ptr(), abcde_l);
            left.extend(store);
            right_value_writer(abcde_r);
        }

        left
    }
}

fn radix_sort<'a>(mut items: &'a mut [u32], mut buf: &'a mut [u32], counts: &mut [u16]) {
    let base = counts.len();
    assert!(base.is_power_of_two());
    let digit_len = base.ilog2();
    for i in 0..2 {
        for &item in items.iter() {
            let digit = (item >> (i * digit_len)) & ((1 << digit_len) - 1);
            counts[digit as usize] += 1;
        }
        for j in 1..counts.len() {
            counts[j] += counts[j - 1];
        }
        for &item in items.iter().rev() {
            let digit = (item >> (i * digit_len)) & ((1 << digit_len) - 1);
            counts[digit as usize] -= 1;
            buf[counts[digit as usize] as usize] = item;
        }

        std::mem::swap(&mut items, &mut buf);
        counts.fill(0);
    }
}

pub fn part_1(input: &str) -> impl Debug {
    let mut right = Vec::with_capacity(1000);
    let mut left = parse_nums(input, |v| {
        let mut store = [0; 4];
        unsafe { vst1q_u32(store.as_mut_ptr(), v) };
        right.extend(store);
    });

    let mut counts = vec![0u16; 512];
    let mut sort_buf = vec![0; left.len()];

    radix_sort(&mut left, &mut sort_buf, &mut counts);
    radix_sort(&mut right, &mut sort_buf, &mut counts);

    left.iter()
        .zip(&right)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
}

pub fn part_2(input: &str) -> impl Debug {
    let input = input.as_bytes();
    let line_len: u8 = 5 + 3 + 5 + 1;
    let line_count = input.len() / line_len as usize;
    assert!(line_count < u16::MAX as usize);
    assert!(line_count % 4 == 0);
    let mut left_bitset = vec![0u64; (1 << (5 * 4)) / 64];
    let mut right_vals: Vec<u32> = Vec::with_capacity(line_count);
    unsafe {
        for chunk in input.chunks_exact(line_len as usize * 4) {
            // We may read out of bounds, but the way we allocate input makes this sound
            let chunk = (chunk.as_ptr() as *const uint8x16x4_t).read_unaligned();

            let idxs = ([
                12,
                10,
                12 + line_len,
                10 + line_len,
                12 + line_len * 2,
                10 + line_len * 2,
                12 + line_len * 3,
                10 + line_len * 3,
                4,
                2,
                4 + line_len,
                2 + line_len,
                4 + line_len * 2,
                2 + line_len * 2,
                4 + line_len * 3,
                2 + line_len * 3,
            ]
            .as_ptr() as *const uint8x16_t)
                .read_unaligned();
            let ac = vqtbl4q_u8(chunk, idxs);

            let idxs = ([
                11,
                9,
                11 + line_len,
                9 + line_len,
                11 + line_len * 2,
                9 + line_len * 2,
                11 + line_len * 3,
                9 + line_len * 3,
                3,
                1,
                3 + line_len,
                1 + line_len,
                3 + line_len * 2,
                1 + line_len * 2,
                3 + line_len * 3,
                1 + line_len * 3,
            ]
            .as_ptr() as *const uint8x16_t)
                .read_unaligned();
            let bd = vqtbl4q_u8(chunk, idxs);
            let abcd = vreinterpretq_u16_u8(vbslq_u8(vdupq_n_u8(0xF), ac, vshlq_n_u8::<4>(bd)));

            let idxs = ([
                8,
                0xFF,
                8 + line_len,
                0xFF,
                8 + line_len * 2,
                0xFF,
                8 + line_len * 3,
                0xFF,
                0,
                0xFF,
                0 + line_len,
                0xFF,
                0 + line_len * 2,
                0xFF,
                0 + line_len * 3,
                0xFF,
            ]
            .as_ptr() as *const uint8x16_t)
                .read_unaligned();
            let e = vreinterpretq_u16_u8(vandq_u8(vqtbl4q_u8(chunk, idxs), vdupq_n_u8(0xF)));

            right_vals.reserve(4);
            vst2_u16(
                right_vals.as_mut_ptr().add(right_vals.len()) as *mut u16,
                uint16x4x2_t(vget_low_u16(abcd), vget_low_u16(e)),
            );
            right_vals.set_len(right_vals.len() + 4);

            let left = vreinterpretq_u32_u16(vzip2q_u16(abcd, e));
            let idx = vgetq_lane_u32::<0>(left) as usize;
            left_bitset[idx / 64] |= 1 << (idx % 64);
            let idx = vgetq_lane_u32::<1>(left) as usize;
            left_bitset[idx / 64] |= 1 << (idx % 64);
            let idx = vgetq_lane_u32::<2>(left) as usize;
            left_bitset[idx / 64] |= 1 << (idx % 64);
            let idx = vgetq_lane_u32::<3>(left) as usize;
            left_bitset[idx / 64] |= 1 << (idx % 64);
        }
    }

    right_vals
        .into_iter()
        .filter_map(|right| {
            (left_bitset[right as usize / 64] & (1 << (right % 64)) != 0).then(|| {
                (right & 0xF)
                    + ((right >> 4) & 0xF) * 10
                    + ((right >> 8) & 0xF) * 100
                    + ((right >> 12) & 0xF) * 1000
                    + ((right >> 16) & 0xF) * 10000
            })
        })
        .sum::<u32>()
}
