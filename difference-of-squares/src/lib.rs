pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // i can do this, but this needs `.unwrap()`, meh
    // (1..=n).reduce(|a, b| a + square(b)).unwrap()

    // fold don't need this, but needs init value
    (1..=n).fold(0, |a, b| a + b.pow(2))

    // map is of course also possible:
    // (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
