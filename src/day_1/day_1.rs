use std::path::Path;
use advent_of_code_2022::read_lines;

pub fn find_max_total_calories() -> i32 {
    let filepath = Path::new("src/day_1/input.txt");
    if let Ok(lines) = read_lines(filepath) {
        let mut current_max_calories = 0;
        let mut current_calorie_count = 0;
        for line in lines {
            if line == "" {
                if current_calorie_count > current_max_calories {
                    current_max_calories = current_calorie_count;
                }
                current_calorie_count = 0;
            } else {
                current_calorie_count += line.parse::<i32>().ok().unwrap();
            }
        }
        return current_max_calories;
    }
    return -1;
} 

