use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Rucksack {
    pub first: String,
    pub second: String,
    pub common_chars: Vec<char>,
}

fn score_char(character: &char) -> i32 {
    let mut priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    (priorities.position(|c| c == *character).unwrap() as i32) + 1
}
impl Rucksack {
    pub fn from_str(input: String) -> Rucksack {
        let len = input.len();
        let (first, _) = input.split_at(len - (len / 2));
        let (_, second) = input.split_at(len / 2);

        let first_set: HashSet<char> = first.chars().collect();
        let common_chars = second
            .chars()
            .filter(|c| first_set.contains(c))
            .unique()
            .collect_vec();

        Rucksack {
            first: String::from(first),
            second: String::from(second),
            common_chars,
        }
    }

    pub fn score(&self) -> i32 {
        self.common_chars.iter().map(score_char).sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElfGroup {
    rucksacks: [Rucksack; 3],
    badge: char,
    score: i32,
}

impl ElfGroup {
    pub fn from_lines(input: [String; 3]) -> ElfGroup {
        let badge = String::from(&input[2])
            .chars()
            .find(|c| input[0].contains(*c) && input[1].contains(*c))
            .unwrap();
        let rucksacks = input.map(Rucksack::from_str);
        let score = score_char(&badge);

        ElfGroup {
            rucksacks,
            badge,
            score,
        }
    }
}

pub fn score_lines(inputs: Vec<String>) -> i32 {
    inputs
        .iter()
        .map(|line| Rucksack::from_str(String::from(line)).score())
        .sum()
}

pub fn group_and_score(inputs: Vec<String>) -> i32 {
    inputs
        .chunks_exact(3)
        .map(|chunk| {
            ElfGroup::from_lines([
                String::from(&chunk[0]),
                String::from(&chunk[1]),
                String::from(&chunk[2]),
            ])
            .score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{ElfGroup, Rucksack};

    #[test]
    fn it_splits_items_into_compartments() {
        let input = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");

        let expected = Rucksack {
            first: String::from("vJrwpWtwJgWr"),
            second: String::from("hcsFMMfFFhFp"),
            common_chars: vec!['p'],
        };

        let actual = Rucksack::from_str(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_splits_an_uneven_list_of_items() {
        let input = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let expected = Rucksack {
            first: String::from("jqHRNqRjqzjGDLGL"),
            second: String::from("rsFMfFZSrLrFZsSL"),
            common_chars: vec!['L'],
        };
        let actual = Rucksack::from_str(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_scores_the_common_items() {
        let inputs = [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let expectations = [16, 38, 42, 22, 20, 19];
        let results = inputs.map(|i| {
            let sack = Rucksack::from_str(i);
            sack.score()
        });

        assert_eq!(expectations, results);
    }

    #[test]
    fn it_calculates_total_line_score() {
        let inputs = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let expected = 157;
        let result = super::score_lines(inputs);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_should_produce_a_group_from_three_lines_of_input() {
        let input = [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ];

        let expected = ElfGroup {
            rucksacks: [
                Rucksack {
                    first: String::from("vJrwpWtwJgWr"),
                    second: String::from("hcsFMMfFFhFp"),
                    common_chars: vec!['p'],
                },
                Rucksack {
                    first: String::from("jqHRNqRjqzjGDLGL"),
                    second: String::from("rsFMfFZSrLrFZsSL"),
                    common_chars: vec!['L'],
                },
                Rucksack {
                    first: String::from("PmmdzqPrV"),
                    second: String::from("vPwwTWBwg"),
                    common_chars: vec!['P'],
                },
            ],
            badge: 'r',
            score: 18,
        };

        let actual = ElfGroup::from_lines(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_groups_and_scores_inputs() {
        let inputs = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        let expected = 70;
        let actual = super::group_and_score(inputs);
        assert_eq!(expected, actual);
    }
}
