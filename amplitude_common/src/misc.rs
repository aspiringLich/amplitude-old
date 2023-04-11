pub fn t<T>(c: bool, a: T, b: T) -> T {
    if c {
        return a;
    }
    b
}
