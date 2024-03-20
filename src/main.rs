mod tests;

use std::io;

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(month: i32, year: i32) -> i32 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn add_days_to_date(day: &mut i32, month: &mut i32, year: &mut i32, mut difference: i32) {
    while difference != 0 {
        let days_this_month = days_in_month(*month, *year);
        if difference + *day > days_this_month {
            difference -= days_this_month - *day + 1;
            *day = 1;
            if *month == 12 {
                *month = 1;
                *year += 1;
            } else {
                *month += 1;
            }
        } else if difference + *day < 1 {
            if *month == 1 {
                *month = 12;
                *year -= 1;
            } else {
                *month -= 1;
            }
            let days_this_month = days_in_month(*month, *year);
            *day = days_this_month + (difference + *day);
            difference = 0;
        } else {
            *day += difference;
            difference = 0;
        }
    }
}

fn main() {
    let mut difference = String::new();
    println!("Enter the difference in days:");
    io::stdin()
        .read_line(&mut difference)
        .expect("Failed to read line");
    let difference: i32 = difference
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut date = String::new();
    println!("Enter a date in the form day month year:");
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");
    let mut parts = date.split_whitespace();
    let day: i32 = parts
        .next()
        .expect("Day is missing")
        .parse()
        .expect("Please enter a valid number");
    let month: i32 = parts
        .next()
        .expect("Month is missing")
        .parse()
        .expect("Please enter a valid number");
    let year: i32 = parts
        .next()
        .expect("Year is missing")
        .parse()
        .expect("Please enter a valid number");

    let mut day = day;
    let mut month = month;
    let mut year = year;

    add_days_to_date(&mut day, &mut month, &mut year, difference);

    println!("The final date is: {}/{}/{}", day, month, year);
}
