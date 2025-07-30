use atlas_common::{CHashOptions, HashFlags, RegisterFunc, Slice};

#[unsafe(no_mangle)]
pub extern "C" fn register_hashers(register: RegisterFunc) {
    register(c"SipHash32 1-3".as_ptr(), HashFlags::KEYED, sip32_1_3);
    register(c"SipHash32 2-4".as_ptr(), HashFlags::KEYED, sip32_2_4);
    register(c"SipHash128 1-3".as_ptr(), HashFlags::KEYED, sip128_1_3);
    register(c"SipHash128 2-4".as_ptr(), HashFlags::KEYED, sip128_2_4);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sip32_1_3(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let (key0, key1) = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<(u64, u64)>()
        } else {
            &(0, 0)
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = siphasher::sip::SipHasher13::new_with_keys(*key0, *key1).hash(input_data);
        let hash_vec = hash.to_be_bytes().to_vec();
        *data = Slice::from_vec(hash_vec);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sip32_2_4(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let (key0, key1) = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<(u64, u64)>()
        } else {
            &(0, 0)
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = siphasher::sip::SipHasher24::new_with_keys(*key0, *key1).hash(input_data);
        let hash_vec = hash.to_be_bytes().to_vec();
        *data = Slice::from_vec(hash_vec);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sip128_1_3(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let (key0, key1) = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<(u64, u64)>()
        } else {
            &(0, 0)
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = siphasher::sip128::SipHasher13::new_with_keys(*key0, *key1).hash(input_data);
        let hash_vec = hash.as_bytes().to_vec();
        *data = Slice::from_vec(hash_vec);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sip128_2_4(data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
    if data.is_null() || options.is_null() {
        return -1;
    }

    unsafe {
        let hash_options = &*options;
        let (key0, key1) = if !hash_options.data.is_null() {
            &*hash_options.data.cast::<(u64, u64)>()
        } else {
            &(0, 0)
        };

        let input = &mut *data;
        let Some(input_data) = input.as_mut_slice() else {
            return -2;
        };

        if !input.ptr.is_null() {
            let _: Option<Box<[u8]>> = input.into_boxed_slice();
        }
        let hash = siphasher::sip128::SipHasher24::new_with_keys(*key0, *key1).hash(input_data);
        let hash_vec = hash.as_bytes().to_vec();
        *data = Slice::from_vec(hash_vec);
    }
    0
}
