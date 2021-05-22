pub fn is_leap_year(year: u64) -> bool {
    // Note:
    //   This does not account for "-" (BC) years,
    //   but it is not designed to do so anyways (using u64)
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
