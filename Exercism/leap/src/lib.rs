pub fn is_leap_year(year: u64) -> bool {
    match year % 4 == 0 && year % 100 != 0 {
        true => true,
        false => return year % 400 == 0,
    }
}
