//! Common types used in Atlas.
//!
//! Provides an FFI friendly boxed ``Slice<T>``, and an opaque data type ``CHashOptions``
//! for sending seeds/secrets to the hashing functions.

use ruint::Uint;

bitflags::bitflags! {
    #[repr(C)]
    pub struct HashFlags: u16 {
        const SEEDED = 0b0000_0001;
        const SECRET = 0b0000_0010;
        const KEYED = 0b0000_0100;
    }
}

#[repr(C)]
pub struct CHashOptions {
    pub data: *const (),
}

#[repr(C)]
pub struct Slice<T> {
    pub ptr: *mut T,
    pub len: usize,
}

impl<T> Slice<T> {
    /// Initializes an FFI-friendly `Slice<T>` from a `Box<[T]>`
    ///
    /// # Examples
    /// ```
    /// use atlas_common::Slice;
    ///
    /// // Make a slice we can send to external functions
    /// let data = vec![0x10, 0x00, 0x40, 0x20];
    /// let slice = Slice::from_boxed_slice(data.into_boxed_slice());
    /// ```
    pub fn from_boxed_slice(boxed_slice: Box<[T]>) -> Self {
        let len = boxed_slice.len();
        let ptr = Box::into_raw(boxed_slice).cast::<T>();
        Self { ptr, len }
    }

    /// Initializes an FFI-friendly `Slice<T>` from a `Vec<T>`
    ///
    /// This function is a wrapper around `from_boxed_slice`,
    /// internally boxing the vec.
    ///
    /// # Examples
    /// ```
    /// use atlas_common::Slice;
    ///
    /// // Make a slice we can send to external functions
    /// let data = vec![0x10, 0x00, 0x40, 0x20];
    /// let slice = Slice::from_vec(data);
    /// ```
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self::from_boxed_slice(vec.into_boxed_slice())
    }

    /// Converts the Slice back into a boxed slice,
    ///
    /// # Examples
    /// ```
    /// use atlas_common::Slice;
    ///
    /// // Make a slice we can send to external functions
    /// let data = vec![0x10, 0x00, 0x40, 0x20];
    /// let slice = Slice::from_vec(data);
    ///
    /// // Perform external function call here that modifies data
    ///
    /// let returned_data = slice.into_boxed_slice();
    /// ```
    pub fn into_boxed_slice(&self) -> Option<Box<[T]>> {
        if self.ptr.is_null() || !self.ptr.is_aligned() || self.len > isize::MAX as usize {
            None
        } else {
            // SAFETY: the pointer is guaranteed to be non-null, aligned, and the len to be less than isize::MAX
            Some(unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.ptr, self.len)) })
        }
    }

    pub fn as_mut_slice(&self) -> Option<&mut [T]> {
        if self.ptr.is_null() || !self.ptr.is_aligned() || self.len > isize::MAX as usize {
            None
        } else {
            // SAFETY: the pointer is guaranteed to be non-null, aligned, and the len to be less than isize::MAX
            Some(unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) })
        }
    }
}

/// The `HashFunction` type defines the function signature that all
/// hashing plugins for Atlas must use.
pub type HashFunction =
    unsafe extern "C" fn(data: *mut Slice<u8>, options: *const CHashOptions) -> i32;

/// The `RegisterFunc` type defines the function signature that Atlas
/// uses to register plugins.
pub type RegisterFunc = extern "C" fn(name: *const i8, flags: HashFlags, hasher: HashFunction);

/// `HashResult` is a wrapper over a string, with helpers to convert from common
/// hashing results.
pub struct HashResult(pub String);

impl HashResult {
    pub fn flip_endian(&self) -> Self {
        let mut data = hex::decode(self.0.clone()).expect("Failed to decode hash result to bytes");
        data.reverse();
        data.into()
    }
}

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

impl<T: std::fmt::UpperHex> From<Box<[T]>> for HashResult {
    fn from(val: Box<[T]>) -> Self {
        let mut res = String::new();
        for b in val {
            res.push_str(&format!("{b:02X}"));
        }
        Self(res)
    }
}
