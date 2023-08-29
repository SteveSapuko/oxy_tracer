pub fn in_range<T: PartialOrd>(x: T, min: T, max: T) -> bool {
    min < x && x < max
}