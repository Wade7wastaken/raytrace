#[inline(always)]
pub fn tern<T>(cond: bool, a: T, b: T) -> T {
    if cond { a } else { b }
}
