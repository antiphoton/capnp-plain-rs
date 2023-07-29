/// TODO: Use `std` after `split_array` feature is stable.
/// https://github.com/rust-lang/rust/issues/90091
#[inline]
#[must_use]
pub fn split_array_ref<T, const N: usize>(data: &[T]) -> (&[T; N], &[T]) {
    let (a, b) = data.split_at(N);
    // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at)
    unsafe { (&*(a.as_ptr() as *const [T; N]), b) }
}
