pub fn is_leap_year(year: u64) -> bool {
    // Note:
    //   This does not account for "-" (BC) years,
    //   but it is not designed to do so anyways (using u64)
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)

    // Note(sprietl): learned about "switch cases"
    // is_leap_year_using_match(year)
    // is_leap_year_using_another_match(year)
}

// Source: https://exercism.io/tracks/rust/exercises/leap/solutions/cf99b53d3ef94b3b8d411c9c53e07eaf
pub fn is_leap_year_using_match(year: u64) -> bool {
    match year % 100 {
        0 => year % 400 == 0,
        _ => year % 4 == 0,
    }
}

// Source: https://exercism.io/tracks/rust/exercises/leap/solutions/3bbc5d781f8d42dc9bc07ea3d6119614
pub fn is_leap_year_using_another_match(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
