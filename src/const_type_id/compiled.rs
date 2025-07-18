use std::any::*;
use std::mem::*;

/// A globally-unique, compile-time derivable identifier for a type.
#[derive(Copy, Clone, Debug)]
pub struct ConstTypeId(TypeId);

impl ConstTypeId {
    /// Gets the ID associated with the given type.
    pub const fn of<T: 'static>() -> Self {
        // Use type name hash instead of TypeId transmutation
        Self {
            hash: Self::const_hash(std::any::type_name::<T>()),
        }
    }

    const fn const_hash(s: &str) -> u64 {
        let bytes = s.as_bytes();
        let mut hash = 0xcbf29ce484222325u64; // FNV offset basis
        let mut i = 0;
        while i < bytes.len() {
            hash ^= bytes[i] as u64;
            hash = hash.wrapping_mul(0x100000001b3); // FNV prime
            i += 1;
        }
        hash
    }

    pub const fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }

    /// Determines whether the two given arrays are equal.
    const fn arrays_eq<const N: usize>(a: &[u8; N], b: &[u8; N]) -> bool {
        let mut i = 0;
        while i < N {
            if a[i] != b[i] {
                return false;
            }

            i += 1;
        }
        true
    }
}

impl From<ConstTypeId> for TypeId {
    fn from(value: ConstTypeId) -> Self {
        value.0
    }
}
