fn square(n: u32) -> u32 {
    u32::pow(n, 2)
}

pub fn square_of_sum(n: u32) -> u32 {
    square((1..n + 1).sum())
}

pub fn sum_of_squares(n: u32) -> u32 {
    // i can do this, but this needs `.unwrap()`, meh
    // (1..n + 1).reduce(|a, b| a + u32::pow(b, 2)).unwrap()

    // fold don't need this, but needs init value
    (1..n + 1).fold(0, |a, b| a + square(b))
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
