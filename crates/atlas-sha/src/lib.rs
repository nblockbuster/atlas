#![allow(dead_code)]

use atlas_common::{CHashOptions, HashFlags, RegisterFunc, Slice};
use paste::paste;
use sha2::Digest as _;

#[unsafe(no_mangle)]
pub extern "C" fn register_hashers(register: RegisterFunc) {
    register(c"SHA1".as_ptr(), HashFlags::empty(), sha1);

    register(c"SHA2-224".as_ptr(), HashFlags::empty(), sha2_224);
    register(c"SHA2-256".as_ptr(), HashFlags::empty(), sha2_256);
    register(c"SHA2-384".as_ptr(), HashFlags::empty(), sha2_384);
    register(c"SHA2-512".as_ptr(), HashFlags::empty(), sha2_512);

    register(c"SHA3-224".as_ptr(), HashFlags::empty(), sha3_224);
    register(c"SHA3-256".as_ptr(), HashFlags::empty(), sha3_256);
    register(c"SHA3-384".as_ptr(), HashFlags::empty(), sha3_384);
    register(c"SHA3-512".as_ptr(), HashFlags::empty(), sha3_512);
}

macro_rules! sha_impl {
    (
        sha_version: $v:expr,
        bits: $bits:expr,
        hasher: $hasher:ty,
    ) => {
        paste! {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [< sha$v _ $bits >](data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
                if data.is_null() {
                    return -1;
                }

                unsafe {
                    let input = &mut *data;
                    let Some(input_data) = input.as_mut_slice() else {
                        return -2;
                    };

                    if !input.ptr.is_null() {
                        let _: Option<Box<[u8]>> = input.into_boxed_slice();
                    }
                    let hash = $hasher::digest(input_data);
                    let hash_boxed = hash.to_vec().into_boxed_slice();
                    *data = Slice::from_boxed_slice(hash_boxed);
                }
                0
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sha1(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() {
        return -1;
    }

    unsafe {
        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = sha1::Sha1::digest(input_data);
        let hash_boxed = hash.to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}
