#[cfg(test)]
use super::*;

#[test]
fn test_is_leap_year() {
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(1900), false);
    assert_eq!(is_leap_year(2024), true);
    assert_eq!(is_leap_year(2021), false);
}

#[test]
fn test_days_in_month() {
    assert_eq!(days_in_month(2, 2020), 29); // Leap year
    assert_eq!(days_in_month(2, 2021), 28); // Non-leap year
    assert_eq!(days_in_month(4, 2021), 30);
    assert_eq!(days_in_month(6, 2021), 30);
    assert_eq!(days_in_month(9, 2021), 30);
    assert_eq!(days_in_month(11, 2021), 30);
    assert_eq!(days_in_month(1, 2021), 31);
    assert_eq!(days_in_month(3, 2021), 31);
    assert_eq!(days_in_month(5, 2021), 31);
    assert_eq!(days_in_month(7, 2021), 31);
    assert_eq!(days_in_month(8, 2021), 31);
    assert_eq!(days_in_month(10, 2021), 31);
    assert_eq!(days_in_month(12, 2021), 31);
}

#[test]
fn test_add_days_to_date() {
    let mut day = 28;
    let mut month = 2;
    let mut year = 2020;
    add_days_to_date(&mut day, &mut month, &mut year, 3);
    assert_eq!(day, 2);
    assert_eq!(month, 3);
    assert_eq!(year, 2020);

    day = 31;
    month = 12;
    year = 2021;
    add_days_to_date(&mut day, &mut month, &mut year, 1);
    assert_eq!(day, 1);
    assert_eq!(month, 1);
    assert_eq!(year, 2022);

    day = 1;
    month = 1;
    year = 2022;
    add_days_to_date(&mut day, &mut month, &mut year, -1);
    assert_eq!(day, 31);
    assert_eq!(month, 12);
    assert_eq!(year, 2021);
}
