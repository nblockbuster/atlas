use atlas_common::{CHashOptions, HashFlags, RegisterFunc, Slice};

#[unsafe(no_mangle)]
pub extern "C" fn register_hashers(register: RegisterFunc) {
    register(c"Murmur2_32".as_ptr(), HashFlags::SEEDED, murmur2_32);
    register(c"Murmur2a_32".as_ptr(), HashFlags::SEEDED, murmur2a_32);
    register(c"Murmur2a_64".as_ptr(), HashFlags::SEEDED, murmur2a_64);
    register(c"Murmur2b_64".as_ptr(), HashFlags::SEEDED, murmur2b_64);
    register(c"Murmur3_32".as_ptr(), HashFlags::SEEDED, murmur3_32);
    register(
        c"Murmur3_x64_128".as_ptr(),
        HashFlags::SEEDED,
        murmur3_x64_128,
    );
    register(
        c"Murmur3_x86_128".as_ptr(),
        HashFlags::SEEDED,
        murmur3_x86_128,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur2_32(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash = murmur2::murmur2(input_data, *seed as u32);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur2a_32(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash = murmur2::murmur2a(input_data, *seed as u32);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur2a_64(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash = murmur2::murmur64a(input_data, *seed);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur2b_64(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash = murmur2::murmur64b(input_data, *seed);
        let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
        *data = Slice::from_boxed_slice(hash_boxed);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur3_32(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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
        let hash_result = murmur3::murmur3_32(&mut std::io::Cursor::new(input_data), *seed as u32);
        match hash_result {
            Ok(hash) => {
                let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
                *data = Slice::from_boxed_slice(hash_boxed);
            }
            Err(e) => {
                eprintln!("murmur3::murmur3_32 returned an error: {e}");
                return -3;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur3_x64_128(
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
        let hash_result =
            murmur3::murmur3_x64_128(&mut std::io::Cursor::new(input_data), *seed as u32);
        // if hash_result.is_err() {
        //     return -1;
        // } else {
        //     let hash_boxed = hash_result
        //         .unwrap()
        //         .to_be_bytes()
        //         .to_vec()
        //         .into_boxed_slice();
        //     *data = Slice::from_boxed_slice(hash_boxed);
        // };
        match hash_result {
            Ok(hash) => {
                let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
                *data = Slice::from_boxed_slice(hash_boxed);
            }
            Err(e) => {
                eprintln!("murmur3::murmur3_x64_128 returned an error: {e}");
                return -3;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn murmur3_x86_128(
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
        let hash_result =
            murmur3::murmur3_x86_128(&mut std::io::Cursor::new(input_data), *seed as u32);
        match hash_result {
            Ok(hash) => {
                let hash_boxed = hash.to_be_bytes().to_vec().into_boxed_slice();
                *data = Slice::from_boxed_slice(hash_boxed);
            }
            Err(e) => {
                eprintln!("murmur3::murmur3_x86_128 returned an error: {e}");
                return -3;
            }
        }
    }
    0
}
