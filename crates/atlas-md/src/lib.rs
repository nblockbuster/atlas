#![allow(dead_code)]
use atlas_common::{CHashOptions, HashFlags, RegisterFunc, Slice};
use md5::Digest as _;
use paste::paste;

#[unsafe(no_mangle)]
pub extern "C" fn register_hashers(register: RegisterFunc) {
    register(c"MD2".as_ptr(), HashFlags::empty(), md2);
    register(c"MD4".as_ptr(), HashFlags::empty(), md4);
    register(c"MD5".as_ptr(), HashFlags::empty(), md5);
}

macro_rules! md_impl {
    (
        version: $v:expr,
        hasher: $hasher:ty,
    ) => {
        paste! {
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [< md$v >](data: *mut Slice<u8>, options: *const CHashOptions) -> i32 {
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

md_impl! {
    version: 2,
    hasher: md2::Md2,
}

md_impl! {
    version: 4,
    hasher: md4::Md4,
}

md_impl! {
    version: 5,
    hasher: md5::Md5,
}
