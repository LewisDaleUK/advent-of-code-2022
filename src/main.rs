use std::fs;
use std::path::Path;

use communication::FindUnique;
use itertools::Itertools;
use rope::RopeBridge;

use crate::trees::{Scenic, Visible};

mod calories;
mod cleaning;
mod communication;
mod directory_parser;
mod file_node;
mod rope;
mod rps;
mod rucksack;
mod stacks;
mod trees;

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
        let overlaps = schedule.split('\n').filter(|a| {
            if a.is_empty() {
                false
            } else {
                cleaning::range_overlaps(a)
            }
        });
        println!(
            "Total cleaning schedules with overlaps: {}",
            overlaps.count()
        );
    }

    if let Some(crane_ops) = read_file(Path::new("./src/inputs/crane_ops.txt")) {
        let mut crane = stacks::Crane::from_str(&crane_ops);
        let output = crane.operate_all();

        println!("{}", output);
    }

    if let Some(comms) = read_file(Path::new("./src/inputs/communication.txt")) {
        if let Some(idx) = comms.find_unique(14) {
            println!("Found marker at position {}", idx);
        }
    }

    if let Some(commands) = read_file(Path::new("./src/inputs/commands.txt")) {
        let mut inputs = commands.split('\n').collect_vec();
        inputs.pop();
        let filesystem = directory_parser::FileSystem::new(inputs);
        let total = filesystem.find_dirs_by_max_size(100000);

        println!("Total size of dirs < 100000: {}", total);
        println!(
            "Smallest dir that can be deleted: {}",
            filesystem.free_size(70000000, 30000000)
        );
    }

    if let Some(tree_data) = read_file(Path::new("./src/inputs/trees.txt")) {
        let inputs = tree_data.lines().collect_vec();
        let grid = trees::construct(inputs);
        println!("There are {} trees visible", grid.count_visible());
        println!("Highest tree score: {}", grid.highest_score());
    }

    if let Some(movements) = read_file(Path::new("./src/inputs/movements.txt")) {
        let lines = movements.lines().collect_vec();
        let mut rope = RopeBridge::default();
        rope.process(lines);

        println!("Made {} moves", rope.tail.visited());
    }
}

fn read_file(path: &Path) -> Option<String> {
    if let Ok(lines) = fs::read_to_string(path) {
        return Some(lines);
    }
    None
}
