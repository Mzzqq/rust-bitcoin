// SPDX-License-Identifier: CC0-1.0

//! Contains `TaprootMerkleBranchBuf` and its associated types.

use hashes::Hash;

use super::{
    InvalidMerkleTreeDepthError, TapNodeHash, TaprootError, TaprootMerkleBranch,
    TAPROOT_CONTROL_MAX_NODE_COUNT,
};
use crate::prelude::{Borrow, BorrowMut, Box, Vec};

/// The Merkle proof for inclusion of a tree in a Taproot tree hash.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "Vec<TapNodeHash>"))]
#[cfg_attr(feature = "serde", serde(try_from = "Vec<TapNodeHash>"))]
pub struct TaprootMerkleBranchBuf(Vec<TapNodeHash>);

impl TaprootMerkleBranchBuf {
    /// Returns a reference to the slice of hashes.
    #[deprecated(since = "0.32.0", note = "use `as_slice` instead")]
    #[inline]
    pub fn as_inner(&self) -> &[TapNodeHash] { &self.0 }

    /// Returns a reference to the slice of hashes.
    #[inline]
    pub fn as_slice(&self) -> &[TapNodeHash] { &self.0 }

    /// Returns a mutable reference to the slice of hashes.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [TapNodeHash] { &mut self.0 }

    /// Returns the number of nodes in this Merkle proof.
    #[inline]
    pub fn len(&self) -> usize { self.0.len() }

    /// Checks if this Merkle proof is empty.
    #[inline]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Decodes bytes from control block.
    ///
    /// This reads the branch as encoded in the control block: the concatenated 32B byte chunks -
    /// one for each hash.
    ///
    /// # Errors
    ///
    /// The function returns an error if the number of bytes is not an integer multiple of 32 or
    /// if the number of hashes exceeds 128.
    pub fn decode(sl: &[u8]) -> Result<Self, TaprootError> {
        TaprootMerkleBranch::decode(sl).map(alloc::borrow::ToOwned::to_owned).map_err(Into::into)
    }

    /// Constructs a new Merkle proof from list of hashes.
    ///
    /// # Errors
    ///
    /// If inner proof length is more than [`TAPROOT_CONTROL_MAX_NODE_COUNT`] (128).
    #[inline]
    fn from_collection<T: AsRef<[TapNodeHash]> + Into<Vec<TapNodeHash>>>(
        collection: T,
    ) -> Result<Self, InvalidMerkleTreeDepthError> {
        if collection.as_ref().len() > TAPROOT_CONTROL_MAX_NODE_COUNT {
            Err(InvalidMerkleTreeDepthError(collection.as_ref().len()))
        } else {
            Ok(TaprootMerkleBranchBuf(collection.into()))
        }
    }

    /// Serializes to a writer.
    ///
    /// # Returns
    ///
    /// The number of bytes written to the writer.
    pub fn encode<Write: io::Write + ?Sized>(&self, writer: &mut Write) -> io::Result<usize> {
        for hash in self {
            writer.write_all(hash.as_ref())?;
        }
        Ok(self.len() * TapNodeHash::LEN)
    }

    /// Serializes `self` as bytes.
    pub fn serialize(&self) -> Vec<u8> {
        self.iter().flat_map(|e| e.as_byte_array()).copied().collect::<Vec<u8>>()
    }

    /// Appends elements to proof.
    pub(in super::super) fn push(
        &mut self,
        h: TapNodeHash,
    ) -> Result<(), InvalidMerkleTreeDepthError> {
        if self.len() >= TAPROOT_CONTROL_MAX_NODE_COUNT {
            Err(InvalidMerkleTreeDepthError(self.0.len()))
        } else {
            self.0.push(h);
            Ok(())
        }
    }

    /// Returns the inner list of hashes.
    #[deprecated(since = "0.32.0", note = "use `into_vec` instead")]
    #[inline]
    pub fn into_inner(self) -> Vec<TapNodeHash> { self.0 }

    /// Returns the list of hashes stored in a `Vec`.
    #[inline]
    pub fn into_vec(self) -> Vec<TapNodeHash> { self.0 }
}

macro_rules! impl_try_from {
    ($from:ty) => {
        impl TryFrom<$from> for TaprootMerkleBranchBuf {
            type Error = InvalidMerkleTreeDepthError;

            /// Constructs a new Merkle proof from list of hashes.
            ///
            /// # Errors
            ///
            /// If inner proof length is more than [`TAPROOT_CONTROL_MAX_NODE_COUNT`] (128).
            #[inline]
            fn try_from(v: $from) -> Result<Self, Self::Error> {
                TaprootMerkleBranchBuf::from_collection(v)
            }
        }
    };
}
impl_try_from!(&[TapNodeHash]);
impl_try_from!(Vec<TapNodeHash>);
impl_try_from!(Box<[TapNodeHash]>);

macro_rules! impl_from_array {
    ($($len:expr),* $(,)?) => {
        $(
            impl From<[TapNodeHash; $len]> for TaprootMerkleBranchBuf {
                #[inline]
                fn from(a: [TapNodeHash; $len]) -> Self {
                    Self(a.to_vec())
                }
            }
        )*
    }
}
// Implement for all values [0, 128] inclusive.
//
// The reason zero is included is that `TaprootMerkleBranchBuf` doesn't contain the hash of the node
// that's being proven - it's not needed because the script is already right before control block.
impl_from_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128
);

impl From<TaprootMerkleBranchBuf> for Vec<TapNodeHash> {
    #[inline]
    fn from(branch: TaprootMerkleBranchBuf) -> Self { branch.0 }
}

impl IntoIterator for TaprootMerkleBranchBuf {
    type IntoIter = IntoIter;
    type Item = TapNodeHash;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { IntoIter(self.0.into_iter()) }
}

impl<'a> IntoIterator for &'a TaprootMerkleBranchBuf {
    type IntoIter = core::slice::Iter<'a, TapNodeHash>;
    type Item = &'a TapNodeHash;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<'a> IntoIterator for &'a mut TaprootMerkleBranchBuf {
    type IntoIter = core::slice::IterMut<'a, TapNodeHash>;
    type Item = &'a mut TapNodeHash;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.0.iter_mut() }
}

impl core::ops::Deref for TaprootMerkleBranchBuf {
    type Target = TaprootMerkleBranch;

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_ref() }
}

impl core::ops::DerefMut for TaprootMerkleBranchBuf {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
}

impl AsRef<[TapNodeHash]> for TaprootMerkleBranchBuf {
    #[inline]
    fn as_ref(&self) -> &[TapNodeHash] { &self.0 }
}

impl AsMut<[TapNodeHash]> for TaprootMerkleBranchBuf {
    #[inline]
    fn as_mut(&mut self) -> &mut [TapNodeHash] { &mut self.0 }
}

impl Borrow<[TapNodeHash]> for TaprootMerkleBranchBuf {
    #[inline]
    fn borrow(&self) -> &[TapNodeHash] { &self.0 }
}

impl BorrowMut<[TapNodeHash]> for TaprootMerkleBranchBuf {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [TapNodeHash] { &mut self.0 }
}

impl<'a> From<&'a TaprootMerkleBranch> for TaprootMerkleBranchBuf {
    fn from(value: &'a TaprootMerkleBranch) -> Self { Self(value.as_slice().into()) }
}

/// Iterator over node hashes within Taproot Merkle branch.
///
/// This is created by `into_iter` method on `TaprootMerkleBranchBuf` (via `IntoIterator` trait).
#[derive(Clone, Debug)]
pub struct IntoIter(alloc::vec::IntoIter<TapNodeHash>);

impl IntoIter {
    /// Returns the remaining items of this iterator as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[TapNodeHash] { self.0.as_slice() }

    /// Returns the remaining items of this iterator as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [TapNodeHash] { self.0.as_mut_slice() }
}

impl Iterator for IntoIter {
    type Item = TapNodeHash;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> { self.0.nth(n) }

    #[inline]
    #[allow(clippy::double_ended_iterator_last)] // False positive.  Using `.last()` on a `vec::IntoIter` is correct.
    fn last(self) -> Option<Self::Item> { self.0.last() }

    #[inline]
    fn count(self) -> usize { self.0.count() }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back() }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> { self.0.nth_back(n) }
}

impl ExactSizeIterator for IntoIter {}

impl core::iter::FusedIterator for IntoIter {}
