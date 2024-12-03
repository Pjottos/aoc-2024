use std::{arch::aarch64::*, fmt::Debug};

pub fn part_1(input: &str) -> impl Debug {
    unsafe { run(input, is_safe) }
}

pub fn part_2(input: &str) -> impl Debug {
    unsafe {
        run(input, |nums, low_len, high_len| {
            let a = is_safe(nums, low_len, high_len);
            let res = if high_len == 0 {
                let low_len = low_len - 1;
                let nums = vcombine_u8(vget_low_u8(nums), vget_low_u8(nums));
                let b = is_safe(remove_nums(nums, 0, 1), low_len, low_len);
                let c = is_safe(remove_nums(nums, 2, 3), low_len, low_len);
                let d = is_safe(remove_nums(nums, 4, 5), low_len, low_len);
                let e = is_safe(remove_nums(nums, 6, 7), low_len, low_len);
                let bcde = vorrq_s64(vorrq_s64(d, e), vorrq_s64(b, c));
                let any_safe = vorr_s64(
                    vorr_s64(vget_low_s64(a), vget_low_s64(bcde)),
                    vget_high_s64(bcde),
                );
                vcombine_s64(any_safe, vdup_n_s64(0))
            } else {
                let mut any_safe = a;
                for i in 0..low_len.max(high_len) {
                    let low_idx = if i < low_len { i } else { 0 };
                    let high_idx = if i < high_len { i } else { 0 };
                    let b = is_safe(
                        remove_nums(nums, low_idx, high_idx),
                        low_len - 1,
                        high_len - 1,
                    );
                    any_safe = vorrq_s64(any_safe, b);
                }

                any_safe
            };

            //let (line, safe_truth) = verification.next().unwrap();
            //assert!(
            //    safe_truth == (vgetq_lane_s64::<0>(res) == -1),
            //    "mismatch {line} {safe_truth} {:02?}",
            //    vget_low_u8(nums),
            //);
            //if high_len != 0 {
            //    let (line, safe_truth) = verification.next().unwrap();
            //    assert!(
            //        safe_truth == (vgetq_lane_s64::<1>(res) == -1),
            //        "mismatch {line} {safe_truth} {:02?}",
            //        vget_high_u8(nums),
            //    );
            //}

            res
        })
    }
}

fn remove_nums(nums: uint8x16_t, low_idx: u32, high_idx: u32) -> uint8x16_t {
    unsafe {
        let shifts = vcombine_s64(
            vcreate_s64((i64::from(low_idx) * -8) as u64),
            vcreate_s64((i64::from(high_idx) * -8) as u64),
        );
        let shifted = vshrq_n_u64::<8>(vreinterpretq_u64_u8(nums));
        let select_mask = vshlq_u64(vdupq_n_u64(u64::MAX), vnegq_s64(shifts));
        //println!("{:016X?}", vreinterpretq_u64_u8(nums));
        //println!("{shifted:016X?}");
        //println!("{select_mask:016X?}");
        let res = vbslq_u8(
            vreinterpretq_u8_u64(select_mask),
            vreinterpretq_u8_u64(shifted),
            nums,
        );
        //println!(
        //    "           {}XX{}XX",
        //    " ".repeat(low_idx as usize * 4),
        //    " ".repeat((8 - low_idx + high_idx) as usize * 4 - 2),
        //);
        //println!("{nums:02X?}");
        //println!("{res:02X?}");
        //println!();
        res
    }
}

unsafe fn run(input: &str, mut safety_check: impl FnMut(uint8x16_t, u32, u32) -> int64x2_t) -> i64 {
    let input = input.as_bytes();
    let mut prev_last_digit = vdupq_n_u8(0);
    let mut nums = vdupq_n_u8(0);
    let mut partial_num_count = 0;
    let mut safe_count = 0;
    for i in (0..input.len()).step_by(16) {
        // Alignment and reading up to 64 bytes past the end ensured by runner
        let chunk = (input.as_ptr().add(i) as *const uint8x16_t).read();
        let digits = vsubq_u8(chunk, vdupq_n_u8(b'0'));
        let digit_mask = vcltq_u8(digits, vdupq_n_u8(10));
        let tens_mask = vandq_u8(digit_mask, vextq_u8::<1>(digit_mask, vdupq_n_u8(0)));

        let sparse_nums = vmlaq_u8(
            digits,
            vextq_u8::<15>(prev_last_digit, vandq_u8(digits, tens_mask)),
            vdupq_n_u8(10),
        );

        let ones_mask;
        let next_byte = input.as_ptr().add(i + 16).read();
        if (b'0'..=b'9').contains(&next_byte) {
            // Last digit is actually in the ten position, so make sure it is not in the ones mask
            // and allow the next iteration to use it with the corresponding one value.
            let last_mask = vsetq_lane_u8::<15>(0xFF, vdupq_n_u8(0));
            prev_last_digit = vandq_u8(digits, vandq_u8(digit_mask, last_mask));
            ones_mask = vbicq_u8(digit_mask, vorrq_u8(tens_mask, last_mask));
        } else {
            prev_last_digit = vdupq_n_u8(0);
            ones_mask = vbicq_u8(digit_mask, tens_mask);
        }

        // There is at least one byte between nums
        let coarse_empty_mask = vceqq_u16(vreinterpretq_u16_u8(ones_mask), vdupq_n_u16(0));
        let bit_extract_mask = vld1q_u16([1u16, 2, 4, 8, 16, 32, 64, 128].as_ptr());
        let lut_idx = !vaddvq_u16(vandq_u16(coarse_empty_mask, bit_extract_mask)) as u8;
        //println!("{lut_idx:08b}");
        let tbl_idxs = COMPACTION_IDXS_LUT[lut_idx as usize];
        //println!("{tbl_idxs:08X?}");
        // Fix the coarseness
        let nums_in_high_bytes = vtstq_u16(vreinterpretq_u16_u8(ones_mask), vdupq_n_u16(0xFF00));
        // Shift right since the shift amount is negative
        let aligned_sparse_nums = vreinterpretq_u8_u16(vshlq_u16(
            vreinterpretq_u16_u8(sparse_nums),
            vreinterpretq_s16_u16(vshlq_n_u16::<3>(nums_in_high_bytes)),
        ));
        //println!("{aligned_sparse_nums:?}");

        let newlines = vceqq_u8(chunk, vdupq_n_u8(b'\n'));
        let newlines_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(
            vreinterpretq_u16_u8(newlines),
        )));
        let ones_bits = vget_lane_u64::<0>(vreinterpret_u64_u8(vshrn_n_u16::<4>(
            vreinterpretq_u16_u8(ones_mask),
        )));

        let newline_count = newlines_bits.count_ones() / 4;
        // We can only store 2 chunks of numbers currently
        assert!(newline_count <= 2);
        let line_end = newlines_bits.trailing_zeros() / 4;
        let chunk_num_count = ones_bits.count_ones() / 4;

        let low_tbl_idxs = if partial_num_count != 0 {
            if partial_num_count != 8 {
                let shift_in = u64::MAX >> (64 - partial_num_count * 8);
                // Most significant lanes are junk if the line has less than 8 nums
                shift_in | (tbl_idxs << (partial_num_count * 8))
            } else {
                u64::MAX
            }
        } else {
            tbl_idxs
        };
        let nums_to_low_count = if newlines_bits != 0 {
            (ones_bits & ((1 << (line_end * 4)) - 1)).count_ones() / 4
        } else {
            (8 - partial_num_count).min(chunk_num_count)
        };
        //println!("{partial_num_count}, {nums_to_low_count}");
        assert!(
            partial_num_count + nums_to_low_count <= 8,
            "Lines with more than 8 nums not supported"
        );
        let nums_to_high_count = chunk_num_count - nums_to_low_count;
        // Most significant lanes will be junk
        let high_tbl_idxs = tbl_idxs >> (nums_to_low_count * 8);
        let full_tbl_idxs = vcombine_u8(vcreate_u8(low_tbl_idxs), vcreate_u8(high_tbl_idxs));
        nums = vqtbx1q_u8(nums, aligned_sparse_nums, full_tbl_idxs);
        partial_num_count += nums_to_low_count;

        if newline_count == 0 {
            continue;
        }

        let high_len = if newline_count == 2 {
            let second_line_end = (64 - newlines_bits.leading_zeros() - 4) / 4;
            let mask = (1 << (second_line_end * 4)) - (1 << (line_end * 4));
            (ones_bits & mask).count_ones() / 4
        } else {
            0
        };
        let is_safe = safety_check(nums, partial_num_count, high_len);
        //let (line, safe_truth) = verification.next().unwrap();
        //assert!(
        //    safe_truth == (vgetq_lane_s64::<0>(is_safe) == -1),
        //    "mismatch {line} {safe_truth} {:02?}",
        //    vget_low_u8(nums),
        //);
        safe_count -= vgetq_lane_s64::<0>(is_safe);
        if newline_count == 2 {
            //let (line, safe_truth) = verification.next().unwrap();
            //assert!(
            //safe_truth == (vgetq_lane_s64::<1>(is_safe) == -1),
            //"mismatch {line} {safe_truth} {:02?}",
            //vget_low_u8(nums),
            //);
            safe_count -= vgetq_lane_s64::<1>(is_safe);
        }
        let third_line_shift = vcreate_s64((-i64::from(high_len)) as u64);
        let high_nums = vshl_u64(vreinterpret_u64_u8(vget_high_u8(nums)), third_line_shift);
        nums = vcombine_u8(vreinterpret_u8_u64(high_nums), vdup_n_u8(0));
        partial_num_count = nums_to_high_count - high_len;
    }

    safe_count
}

fn is_safe(nums: uint8x16_t, low_len: u32, high_len: u32) -> int64x2_t {
    unsafe {
        let diffs = vreinterpretq_s8_u8(vsubq_u8(
            vreinterpretq_u8_u64(vshrq_n_u64::<8>(vreinterpretq_u64_u8(nums))),
            nums,
        ));

        // Masks out padding nums and the last lane
        let partial_shifts = vcombine_s64(
            vcreate_s64(-((1 + 8 - i64::from(low_len)) * 8) as u64),
            vcreate_s64(-((1 + 8 - i64::from(high_len)) * 8) as u64),
        );
        let any_zero = vceqq_s8(diffs, vdupq_n_s8(0));
        let any_zero = vreinterpretq_u64_u8(any_zero);
        let any_zero = vtstq_u64(any_zero, vshlq_u64(vdupq_n_u64(u64::MAX), partial_shifts));
        //println!("{any_zero:02?}");

        let shifted_diffs = vreinterpretq_s8_u64(vshrq_n_u64::<8>(vreinterpretq_u64_s8(diffs)));
        let non_monotonic = veorq_s8(diffs, shifted_diffs);
        let non_monotonic = vreinterpretq_u64_s8(non_monotonic);
        let non_monotonic = vtstq_u64(
            non_monotonic,
            vshlq_u64(vdupq_n_u64(0x00808080_80808080), partial_shifts),
        );
        //println!("{non_monotonic:02X?}");

        let out_bounds = vcgtq_u8(vreinterpretq_u8_s8(vabsq_s8(diffs)), vdupq_n_u8(3));
        let out_bounds = vreinterpretq_u64_u8(out_bounds);
        let out_bounds = vtstq_u64(out_bounds, vshlq_u64(vdupq_n_u64(u64::MAX), partial_shifts));
        //println!("{out_bounds:02?}");

        let is_unsafe = vorrq_u64(out_bounds, vorrq_u64(non_monotonic, any_zero));
        vreinterpretq_s64_u64(vceqzq_u64(is_unsafe))
    }
}

const COMPACTION_IDXS_LUT: [u64; 0x100] = compaction_idxs_lut();

const fn compaction_idxs_lut() -> [u64; 0x100] {
    let mut res = [u64::MAX; 0x100];
    let mut i = 0;
    while i < 0x100 {
        let mut j = 0;
        let mut idx = 0;
        while j < 8 {
            if i & (1 << j) != 0 {
                res[i] &= !(0xFF << idx);
                res[i] |= (j * 2) << idx;
                idx += 8;
            }
            j += 1;
        }
        i += 1;
    }

    res
}
