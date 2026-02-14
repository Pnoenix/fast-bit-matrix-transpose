#![feature(portable_simd)]
use std::simd::prelude::*;


fn main() {
    
}


fn transpose(matrix: &mut[u32; 32]) {
    // 16 bits
    let hi = u32x16::from_slice(&matrix[00..16]);
    let lo = u32x16::from_slice(&matrix[16..32]);

    const MASK_16:  u32x16 = u32x16::splat(0xFFFF0000);
    const SHIFT_16: u32x16 = u32x16::splat(16);

    let hi16 = (hi &  MASK_16) | lo >> SHIFT_16;
    let lo16 = (lo & !MASK_16) | hi << SHIFT_16;

    // 8 bits
    const SWIZZLE_8_HI: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23];
    const SWIZZLE_8_LO: [usize; 16] = [8, 9, 10, 11, 12, 13, 14, 15, 24, 25, 26, 27, 28, 29, 30, 31];
    let hi8_prep = simd_swizzle!(hi16, lo16, SWIZZLE_8_HI);
    let lo8_prep = simd_swizzle!(hi16, lo16, SWIZZLE_8_LO);

    const MASK_8:  u32x16 = u32x16::splat(0xFF00FF00);
    const SHIFT_8: u32x16 = u32x16::splat(8);

    let hi8 = (hi8_prep &  MASK_8) | ( (lo8_prep &  MASK_8) >> SHIFT_8 );
    let lo8 = (lo8_prep & !MASK_8) | ( (hi8_prep & !MASK_8) << SHIFT_8 );

    // 4 bits
    const SWIZZLE_4_HI: [usize; 16] = [0, 1, 2, 3, 16, 17, 18, 19, 8, 9, 10, 11, 24, 25, 26, 27];
    const SWIZZLE_4_LO: [usize; 16] = [4, 5, 6, 7, 20, 21, 22, 23, 12, 13, 14, 15, 28, 29, 30, 31];
    let hi4_prep = simd_swizzle!(hi8, lo8, SWIZZLE_4_HI);
    let lo4_prep = simd_swizzle!(hi8, lo8, SWIZZLE_4_LO);

    const MASK_4:  u32x16 = u32x16::splat(0xF0F0F0F0);
    const SHIFT_4: u32x16 = u32x16::splat(4);

    let hi4 = (hi4_prep &  MASK_4) | ( (lo4_prep &  MASK_4) >> SHIFT_4 );
    let lo4 = (lo4_prep & !MASK_4) | ( (hi4_prep & !MASK_4) << SHIFT_4 );

    // 2 bits
    const SWIZZLE_2_HI: [usize; 16] = [0, 1, 16, 17, 4, 5, 20, 21, 8, 9, 24, 25, 12, 13, 28, 29];
    const SWIZZLE_2_LO: [usize; 16] = [2, 3, 18, 19, 6, 7, 22, 23, 10, 11, 26, 27, 14, 15, 30, 31];
    let hi2_prep = simd_swizzle!(hi4, lo4, SWIZZLE_2_HI);
    let lo2_prep = simd_swizzle!(hi4, lo4, SWIZZLE_2_LO);

    const MASK_2:  u32x16 = u32x16::splat(0xCCCCCCCC);
    const SHIFT_2: u32x16 = u32x16::splat(2);

    let hi2 = (hi2_prep &  MASK_2) | ( (lo2_prep &  MASK_2) >> SHIFT_2 );
    let lo2 = (lo2_prep & !MASK_2) | ( (hi2_prep & !MASK_2) << SHIFT_2 );

    // 1 bit
    const SWIZZLE_1_HI: [usize; 16] = [0, 16, 2, 18, 4, 20, 6, 22, 8, 24, 10, 26, 12, 28, 14, 30];
    const SWIZZLE_1_LO: [usize; 16] = [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31];
    let hi1_prep = simd_swizzle!(hi2, lo2, SWIZZLE_1_HI);
    let lo1_prep = simd_swizzle!(hi2, lo2, SWIZZLE_1_LO);

    const MASK_1:  u32x16 = u32x16::splat(0xAAAAAAAA);
    const SHIFT_1: u32x16 = u32x16::splat(1);

    let hi1 = (hi1_prep &  MASK_1) | ( (lo1_prep &  MASK_1) >> SHIFT_1 );
    let lo1 = (lo1_prep & !MASK_1) | ( (hi1_prep & !MASK_1) << SHIFT_1 );

    // Final swizzle
    const SWIZZLE_F_HI: [usize; 16] = [0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23];
    const SWIZZLE_F_LO: [usize; 16] = [8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31];
    let final_hi = simd_swizzle!(hi1, lo1, SWIZZLE_F_HI);
    let final_lo = simd_swizzle!(hi1, lo1, SWIZZLE_F_LO);

    final_hi.copy_to_slice(&mut matrix[00..16]);
    final_lo.copy_to_slice(&mut matrix[16..32]);
}
