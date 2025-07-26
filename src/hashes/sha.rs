use super::HashResult;
use paste::paste;
use sha2::Digest;

macro_rules! sha_impl {
    (
        sha_version: $v:expr,
        bits: $bits:expr,
        hasher: $hasher:ty,
    ) => {
        paste! {
            #[inline(always)]
            pub fn [< sha$v _ $bits >](data: &[u8]) -> HashResult {
                $hasher::digest(data).as_slice().into()
            }
        }
    };
}

// SHA2-224
sha_impl! {
    sha_version: 2,
    bits: 224,
    hasher: sha2::Sha224,
}

// SHA2-256
sha_impl! {
    sha_version: 2,
    bits: 256,
    hasher: sha2::Sha256,
}

// SHA2-384
sha_impl! {
    sha_version: 2,
    bits: 384,
    hasher: sha2::Sha384,
}

// SHA2-512
sha_impl! {
    sha_version: 2,
    bits: 512,
    hasher: sha2::Sha512,
}

// SHA3-224
sha_impl! {
    sha_version: 3,
    bits: 224,
    hasher: sha3::Sha3_224,
}

// SHA3-256
sha_impl! {
    sha_version: 3,
    bits: 256,
    hasher: sha3::Sha3_256,
}

// SHA3-384
sha_impl! {
    sha_version: 3,
    bits: 384,
    hasher: sha3::Sha3_384,
}

// SHA3-512
sha_impl! {
    sha_version: 3,
    bits: 512,
    hasher: sha3::Sha3_512,
}

pub fn sha1(data: &[u8]) -> HashResult {
    sha1::Sha1::digest(data).as_slice().into()
}
