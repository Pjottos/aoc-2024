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

pub fn part_1(input: &str) -> impl Debug {
    let mut right = Vec::with_capacity(1000);
    let mut left = parse_nums(input, |v| {
        let mut store = [0; 4];
        unsafe { vst1q_u32(store.as_mut_ptr(), v) };
        right.extend(store);
    });

    const DIGIT_LEN: usize = 9;
    let mut counts = vec![0u16; 1 << DIGIT_LEN];
    let mut sort_buf = vec![0; left.len()];

    fn radix_sort<'a>(mut items: &'a mut [u32], mut buf: &'a mut [u32], counts: &mut [u16]) {
        for i in 0..2 {
            for &item in items.iter() {
                let digit = (item >> (i * DIGIT_LEN)) & ((1 << DIGIT_LEN) - 1);
                counts[digit as usize] += 1;
            }
            for j in 1..counts.len() {
                counts[j] += counts[j - 1];
            }
            for &item in items.iter().rev() {
                let digit = (item >> (i * DIGIT_LEN)) & ((1 << DIGIT_LEN) - 1);
                counts[digit as usize] -= 1;
                buf[counts[digit as usize] as usize] = item;
            }

            std::mem::swap(&mut items, &mut buf);
            counts.fill(0);
        }
    }

    radix_sort(&mut left, &mut sort_buf, &mut counts);
    radix_sort(&mut right, &mut sort_buf, &mut counts);

    left.iter()
        .zip(&right)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<u32>()
}

pub fn part_2(input: &str) -> impl Debug {
    let mut right_counts = vec![0u16; 100000];
    let left = parse_nums(input, |v| unsafe {
        right_counts[vgetq_lane_u32::<0>(v) as usize] += 1;
        right_counts[vgetq_lane_u32::<1>(v) as usize] += 1;
        right_counts[vgetq_lane_u32::<2>(v) as usize] += 1;
        right_counts[vgetq_lane_u32::<3>(v) as usize] += 1;
    });

    let mut similarity_score = 0;
    for l in left {
        similarity_score += l * u32::from(right_counts[l as usize]);
    }

    similarity_score
}
