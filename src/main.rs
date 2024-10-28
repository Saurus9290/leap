pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() {
    let years = vec![2000, 1900, 2024, 2023];
    
    for year in years {
        if is_leap_year(year) {
            println!("{} is a leap year.", year);
        } else {
            println!("{} is not a leap year.", year);
        }
    }
}
