use ruint::Uint;
use sha1::Digest as _;

pub mod fnv;
pub mod murmur;
pub mod sha;
pub mod xxhash;

// TODO: Hasher trait
// TODO: (after hasher trait) implement seeds (murmur, xxhash)
// TODO: Dynamically load list of all algorithms (anything implementing Hasher) (todo: how???)
// TODO: should rearrange algorithms so that fnv0-32, fnv0-64, fnv0-128, etc., fnv1-32, fnv1-64, etc.

#[derive(Clone, Debug, Default)]
pub struct HashOptions {}

#[expect(non_camel_case_types)]
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter,
)]
pub enum HashingAlgorithm {
    Fnv0_32,
    Fnv1_32,
    Fnv1a_32,
    Fnv0_64,
    Fnv1_64,
    Fnv1a_64,
    Fnv0_128,
    Fnv1_128,
    Fnv1a_128,
    Fnv0_256,
    Fnv1_256,
    Fnv1a_256,
    Fnv0_512,
    Fnv1_512,
    Fnv1a_512,
    Fnv0_1024,
    Fnv1_1024,
    Fnv1a_1024,

    Md5,

    Murmur2_32,
    Murmur2a_32,
    Murmur2a_64,
    Murmur2b_64,

    SipHash,

    Sha1,

    Sha2_224,
    Sha2_256,
    Sha2_384,
    Sha2_512,

    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,

    XxHash32,
    XxHash64,
    XxHash3_64,
    XxHash3_128,

    Whirlpool,
}

impl HashingAlgorithm {
    pub fn hasher(&self) -> impl Fn(&[u8]) -> HashResult {
        match self {
            Self::Fnv0_32 => fnv::fnv0_32,
            Self::Fnv0_64 => fnv::fnv0_64,
            Self::Fnv0_128 => fnv::fnv0_128,
            Self::Fnv0_256 => fnv::fnv0_256,
            Self::Fnv0_512 => fnv::fnv0_512,
            Self::Fnv0_1024 => fnv::fnv0_1024,

            Self::Fnv1_32 => fnv::fnv1_32,
            Self::Fnv1_64 => fnv::fnv1_64,
            Self::Fnv1_128 => fnv::fnv1_128,
            Self::Fnv1_256 => fnv::fnv1_256,
            Self::Fnv1_512 => fnv::fnv1_512,
            Self::Fnv1_1024 => fnv::fnv1_1024,

            Self::Fnv1a_32 => fnv::fnv1a_32,
            Self::Fnv1a_64 => fnv::fnv1a_64,
            Self::Fnv1a_128 => fnv::fnv1a_128,
            Self::Fnv1a_256 => fnv::fnv1a_256,
            Self::Fnv1a_512 => fnv::fnv1a_512,
            Self::Fnv1a_1024 => fnv::fnv1a_1024,

            Self::Md5 => |data: &[u8]| -> HashResult { md5::compute(data).0.into() },

            Self::Murmur2_32 => murmur::murmur2_32,
            Self::Murmur2a_32 => murmur::murmur2a_32,
            Self::Murmur2a_64 => murmur::murmur2a_64,
            Self::Murmur2b_64 => murmur::murmur2b_64,

            Self::SipHash => |data: &[u8]| -> HashResult {
                let mut hasher = std::hash::DefaultHasher::new();
                std::hash::Hasher::write(&mut hasher, data);
                std::hash::Hasher::finish(&hasher).into()
            },

            Self::Sha1 => sha::sha1,
            Self::Sha2_224 => sha::sha2_224,
            Self::Sha2_256 => sha::sha2_256,
            Self::Sha2_384 => sha::sha2_384,
            Self::Sha2_512 => sha::sha2_512,

            Self::Sha3_224 => sha::sha3_224,
            Self::Sha3_256 => sha::sha3_256,
            Self::Sha3_384 => sha::sha3_384,
            Self::Sha3_512 => sha::sha3_512,

            Self::XxHash32 => xxhash::xxh32,
            Self::XxHash64 => xxhash::xxh64,
            Self::XxHash3_64 => xxhash::xxh3_64,
            Self::XxHash3_128 => xxhash::xxh3_128,

            Self::Whirlpool => {
                |data: &[u8]| -> HashResult { whirlpool::Whirlpool::digest(data).as_slice().into() }
            }
        }
    }
}

pub struct HashResult(pub String);

impl From<u32> for HashResult {
    fn from(val: u32) -> Self {
        Self(format!("{val:02X}"))
    }
}

impl From<u64> for HashResult {
    fn from(val: u64) -> Self {
        Self(format!("{val:02X}"))
    }
}

impl From<u128> for HashResult {
    fn from(val: u128) -> Self {
        Self(format!("{val:02X}"))
    }
}

impl<const B: usize, const L: usize> From<Uint<B, L>> for HashResult {
    fn from(value: Uint<B, L>) -> Self {
        Self(format!("{value:02X}"))
    }
}

impl<T: std::fmt::UpperHex> From<&[T]> for HashResult {
    fn from(val: &[T]) -> Self {
        let mut res = String::new();
        for b in val {
            res.push_str(&format!("{b:02X}"));
        }
        Self(res)
    }
}

impl<T: std::fmt::UpperHex> From<Vec<T>> for HashResult {
    fn from(val: Vec<T>) -> Self {
        let mut res = String::new();
        for b in val {
            res.push_str(&format!("{b:02X}"));
        }
        Self(res)
    }
}

impl<T: std::fmt::UpperHex, const L: usize> From<[T; L]> for HashResult {
    fn from(val: [T; L]) -> Self {
        let mut res = String::new();
        for b in val {
            res.push_str(&format!("{b:02X}"));
        }
        Self(res)
    }
}

pub fn flip_endian(hash: HashResult) -> HashResult {
    let mut data = hex::decode(hash.0).expect("Failed to decode hash result to bytes");
    data.reverse();
    data.into()
}
