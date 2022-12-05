use std::fs;
use std::path::Path;

mod calories;
mod cleaning;
mod rps;
mod rucksack;
mod stacks;

fn main() {
    if let Some(calorie_lines) = read_file(Path::new("./src/inputs/calories.txt")) {
        let mut cals = calories::Calories::from_str(&calorie_lines);
        let max = cals.max();
        println!("Most calories: {}", max);
        println!("Total of top 3 elves: {}", cals.total_n(3));
    }

    if let Some(rock_paper_scissors) = read_file(Path::new("./src/inputs/rock-paper-scissors.txt"))
    {
        let game = rps::Game::from_str(&rock_paper_scissors);
        println!("Total score: {}", game.total_score());
    }

    if let Some(rucksacks) = read_file(Path::new("./src/inputs/rucksacks.txt")) {
        let total_score = rucksack::score_lines(rucksacks.split('\n').map(String::from).collect());
        let badge_score =
            rucksack::group_and_score(rucksacks.split('\n').map(String::from).collect());
        println!("Total elf score: {}", total_score);
        println!("Total badge score: {}", badge_score);
    }

    if let Some(schedule) = read_file(Path::new("./src/inputs/cleaning-schedule.txt")) {
        let overlaps = schedule
            .split('\n')
            .filter(|a| {
                if a.is_empty() {
                    false
                } else {
                    cleaning::range_overlaps(a)
                }
            });
        println!("Total cleaning schedules with overlaps: {}", overlaps.count());
    }

    if let Some(crane_ops) = read_file(Path::new("./src/inputs/crane_ops.txt")) {
        let mut crane = stacks::Crane::from_str(&crane_ops);
        let output = crane.operate_all();

        println!("{:#?}", crane.stacks);
        println!("{}", output);
    }
}

fn read_file(path: &Path) -> Option<String> {
    if let Ok(lines) = fs::read_to_string(path) {
        return Some(lines);
    }
    None
}
