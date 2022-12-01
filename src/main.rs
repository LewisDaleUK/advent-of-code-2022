use std::fs;
use std::path::{Path};

mod calories;

fn main() {
    if let Some(calorie_lines) = read_file(Path::new("./src/inputs/calories.txt")) {
        let mut cals = calories::Calories::from_str(&calorie_lines);
        let max = cals.max();
        println!("Most calories: {}", max);
        println!("Total of top 3 elves: {}", cals.total_n(3));
    }
}

fn read_file(path: &Path) -> Option<String> {
    if let Ok(lines) = fs::read_to_string(path) {
        return Some(lines);
    }
    None
}
