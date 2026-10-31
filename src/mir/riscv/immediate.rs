pub fn fits_imm12(imm: i64) -> bool {
    (-2048..=2047).contains(&imm)
}

pub fn fits_jal_imm(imm: i64) -> bool {
    (imm & 1 == 0) && (-(1 << 20)..(1 << 20)).contains(&imm)
}

pub fn fits_branch_imm(imm: i64) -> bool {
    (imm & 1 == 0) && (-(1 << 12)..(1 << 12)).contains(&imm)
}