pub fn fits_pair_mem_inst(offset: i32) -> bool {
    (-512..=504).contains(&offset)
}

pub fn fits_mem_imm_x(offset: i32) -> bool {
    (0..=32760).contains(&offset) && offset % 8 == 0
}

pub fn fits_mem_imm_w(offset: i32) -> bool {
    (0..=16380).contains(&offset) && offset % 4 == 0
}

pub fn fits_add_sub_imm12(imm: i64) -> bool {
    ((-4095..=4096).contains(&imm)) || (imm % 0x1000 == 0 && (imm >> 12) < 4096)
}

pub fn fits_bitmask_imm(_imm: u64) -> bool {
    // TODO: 完整实现按 ARM ARM 规则检查重复模式
    false
}