use super::HashResult;

#[inline(always)]
pub fn xxh32(data: &[u8]) -> HashResult {
    xxhash_rust::xxh32::xxh32(data, 0).into()
}

#[inline(always)]
pub fn xxh64(data: &[u8]) -> HashResult {
    xxhash_rust::xxh64::xxh64(data, 0).into()
}

#[inline(always)]
pub fn xxh3_64(data: &[u8]) -> HashResult {
    xxhash_rust::xxh3::xxh3_64(data).into()
}

#[inline(always)]
pub fn xxh3_128(data: &[u8]) -> HashResult {
    xxhash_rust::xxh3::xxh3_128(data).into()
}
