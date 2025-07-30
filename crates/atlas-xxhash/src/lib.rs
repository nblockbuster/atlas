use atlas_common::{CHashOptions, HashFlags, RegisterFunc, Slice};

#[unsafe(no_mangle)]
pub extern "C" fn register_hashers(register: RegisterFunc) {
    register(c"XXH32".as_ptr(), HashFlags::SEEDED, xxh32);
    register(c"XXH64".as_ptr(), HashFlags::SEEDED, xxh64);
    register(c"XXH3_64".as_ptr(), HashFlags::empty(), xxh3_64);
    register(
        c"XXH3_64_seeded".as_ptr(),
        HashFlags::SEEDED,
        xxh3_64_seeded,
    );
    // register(
    //     c"XXH3_64_with_secret".as_ptr(),
    //     HashFlags::SECRET,
    //     xxh3_64_seeded,
    // );
    register(c"XXH3_128".as_ptr(), HashFlags::empty(), xxh3_128);
    register(
        c"XXH3_128_seeded".as_ptr(),
        HashFlags::SEEDED,
        xxh3_128_seeded,
    );
    // register(
    //     c"XXH3_128_with_secret".as_ptr(),
    //     HashFlags::SECRET,
    //     xxh3_128_with_secret,
    // );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh32(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let seed = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<u64>()
        } else {
            &0
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = xxhash_rust::xxh32::xxh32(input_data, *seed as u32);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh64(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let seed = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<u64>()
        } else {
            &0
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = xxhash_rust::xxh64::xxh64(input_data, *seed);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh3_64(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash = xxhash_rust::xxh3::xxh3_64(input_data);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh3_64_seeded(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let seed = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<u64>()
        } else {
            &0
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = xxhash_rust::xxh3::xxh3_64_with_seed(input_data, *seed);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh3_64_with_secret(
    data: *mut Slice<u8>,
    options: *const CHashOptions,
) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let secret = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<Slice<u8>>()
            // Box::from_raw(hash_options.data as *mut Slice<u8>)
        } else {
            &Slice::from_vec(vec![0u8])
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }

        let Some(secret_data) = secret.as_mut_slice() else {
            return -2;
        };
        if !secret.ptr.is_null() {
            let _: Option<Box<[u8]>> = secret.into_boxed_slice();
        }

        let hash = xxhash_rust::xxh3::xxh3_64_with_secret(input_data, secret_data);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh3_128(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash = xxhash_rust::xxh3::xxh3_128(input_data);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh3_128_seeded(
    data: *mut Slice<u8>,
    options: *const CHashOptions,
) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let seed = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<u64>()
        } else {
            &0
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = xxhash_rust::xxh3::xxh3_128_with_seed(input_data, *seed);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xxh3_128_with_secret(
    data: *mut Slice<u8>,
    options: *const CHashOptions,
) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let secret = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<Slice<u8>>()
        } else {
            &Slice::from_vec(vec![0u8])
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };
        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }

        let Some(secret_data) = secret.as_mut_slice() else {
            return -2;
        };
        if !secret.ptr.is_null() {
            let _: Option<Box<[u8]>> = secret.into_boxed_slice();
        }

        let hash = xxhash_rust::xxh3::xxh3_128_with_secret(input_data, secret_data);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}
