use std::mem::{transmute_copy, MaybeUninit};

/// Create a new array of `MaybeUninit<T>` items, in an uninitialized state.
///
/// Note: in a future Rust version this method may become unnecessary
/// when Rust allows
/// [inline const expressions](https://github.com/rust-lang/rust/issues/76001).
/// The example below could then use `let mut buf = [const { MaybeUninit::<u8>::uninit() }; 32];`.
///
/// # Examples
///
/// ```no_run
///
/// use std::mem::MaybeUninit;
/// use verge::mem::maybe_uninit;
///
/// extern "C" {
///     fn read_into_buffer(ptr: *mut u8, max_len: usize) -> usize;
/// }
///
/// /// Returns a (possibly smaller) slice of data that was actually read
/// fn read(buf: &mut [MaybeUninit<u8>]) -> &[u8] {
///     unsafe {
///         let len = read_into_buffer(buf.as_mut_ptr() as *mut u8, buf.len());
///         maybe_uninit::slice_assume_init_ref(&buf[..len])
///     }
/// }
///
/// let mut buf: [MaybeUninit<u8>; 32] = maybe_uninit::uninit_array();
/// let data = read(&mut buf);
/// ```
#[cfg(feature = "maybe_uninit_uninit_array")]
pub const fn uninit_array<const N: usize, T>() -> [MaybeUninit<T>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

/// Transposes a `MaybeUninit<[T; N]>` into a `[MaybeUninit<T>; N]`.
/// Based on [std::mem::MaybeUninit::transpose].
///
/// # Examples
///
/// ```
/// # use std::mem::MaybeUninit;
/// # use verge::mem::maybe_uninit;
///
/// let data: [MaybeUninit<u8>; 1000] = maybe_uninit::transpose(MaybeUninit::uninit());
///
/// ```
///
/// # Differences
///
/// Not const because the value may contain interior mutability see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information.
#[cfg(feature = "maybe_uninit_uninit_array_transpose")]
#[inline]
pub fn transpose<T, const N: usize>(value: MaybeUninit<[T; N]>) -> [MaybeUninit<T>; N] {
    // SAFETY: T and MaybeUninit<T> have the same layout
    unsafe { transmute_copy(&value) }
}

/// Assuming all the elements are initialized, get a slice to them.
///
/// # Safety
///
/// It is up to the caller to guarantee that the `MaybeUninit<T>` elements
/// really are in an initialized state.
/// Calling this when the content is not yet fully initialized causes undefined behavior.
///
/// See [`assume_init_ref`] for more details and examples.
///
/// [`assume_init_ref`]: MaybeUninit::assume_init_ref
#[cfg(feature = "maybe_uninit_slice")]
#[inline(always)]
pub const unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    // SAFETY: casting `slice` to a `*const [T]` is safe since the caller guarantees that
    // `slice` is initialized, and `MaybeUninit` is guaranteed to have the same layout as `T`.
    // The pointer obtained is valid since it refers to memory owned by `slice` which is a
    // reference and thus guaranteed to be valid for reads.
    unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
}
