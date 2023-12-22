pub fn extend_and_get<T: Default>(a: &mut Vec<T>, i: usize) -> &mut T {
    if i < a.len() {
        &mut a[i]
    } else {
        a.resize_with(i + 1, Default::default);
        a.last_mut().unwrap()
    }
}
