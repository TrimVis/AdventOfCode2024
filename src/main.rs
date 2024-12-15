mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

use colored::Colorize;

fn get_day_solve_fn(day: &str) -> Option<fn()> {
    match day {
        "1" => Some(day1::solve),
        "2" => Some(day2::solve),
        "3" => Some(day3::solve),
        "4" => Some(day4::solve),
        "5" => Some(day5::solve),
        "6" => Some(day6::solve),
        "7" => Some(day7::solve),
        "8" => Some(day8::solve),
        "9" => Some(day9::solve),
        "10" => Some(day10::solve),
        "11" => Some(day11::solve),
        "12" => Some(day12::solve),
        "13" => Some(day13::solve),
        "14" => Some(day14::solve),
        "15" => Some(day15::solve),
        "16" => None,
        "17" => None,
        "18" => None,
        "19" => None,
        "20" => None,
        "21" => None,
        "22" => None,
        "23" => None,
        "24" => None,
        _ => unimplemented!("The day '{}' is not supported", day),
    }
}

fn main() {
    let mut days: Vec<String> = std::env::args().skip(1).collect();
    if days.len() == 0 {
        days = (1..=24).map(|v| v.to_string()).collect();
    }

    for day in days {
        if let Some(func) = get_day_solve_fn(day.as_str()) {
            let title = format!("{} {}", "Day".yellow(), day.green());
            println!("\n{}", title.bold());
            let start = std::time::Instant::now();
            func();
            let duration = start.elapsed();
            let time = format!("Executed in {:?}", duration);
            println!("{}", time.dimmed());
        } else {
            println!("Day {} not yet implemented!", day);
        }
    }
}
