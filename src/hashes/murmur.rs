use super::HashResult;

#[inline(always)]
pub fn murmur2_32(data: &[u8]) -> HashResult {
    murmur2::murmur2(data, 0).into()
}

#[inline(always)]
pub fn murmur2a_32(data: &[u8]) -> HashResult {
    murmur2::murmur2a(data, 0).into()
}

#[inline(always)]
pub fn murmur2a_64(data: &[u8]) -> HashResult {
    murmur2::murmur64a(data, 0).into()
}

#[inline(always)]
pub fn murmur2b_64(data: &[u8]) -> HashResult {
    murmur2::murmur64b(data, 0).into()
}
