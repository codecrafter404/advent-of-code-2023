use days::{Chellange, day_1::DayOne};

use crate::days::{day_2::DayTwo, day_3::DayThree};

pub mod days;


fn main() -> color_eyre::Result<()> {
    let days: Vec<Box<dyn Chellange>> = vec![
        Box::new(DayOne {}),
        Box::new(DayTwo {}),
        Box::new(DayThree {}),
    ];

    let current_day: u8 = 3;

    let day = days.iter().filter(|x| x.get_day() == current_day).last().expect("Invalid day");
    
    let input = std::fs::read_to_string(format!("./input/day_{}_a.txt", current_day))?;
    println!("Solution A for day {}: {}", current_day, day.solve_first(&input)?);

    let input = std::fs::read_to_string(format!("./input/day_{}_b.txt", current_day))?;
    println!("Solution B for day {}: {}", current_day, day.solve_second(&input)?);


    Ok(())
}
