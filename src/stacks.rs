use std::collections::VecDeque;
use regex::Regex;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Operation {
	quantity: usize,
	source: usize,
	target: usize
} 

#[derive(Debug, PartialEq, Eq)]
pub struct Crane {
	pub stacks: Vec<String>,
	operations: VecDeque<Operation>,
}

trait Take<T> {
	fn take(&mut self, n: usize) -> Option<T>;
}

trait Prepend {
	fn prepend(&mut self, s: &str);
}

impl Take<String> for String {
	fn take(&mut self, n: usize) -> Option<String> {
		println!("Taking {} chars from {}", n, self);
		if n <= self.len() {
			let split = String::from(&self[..n]);
			self.replace_range(..n, "");
			return Some(split);
		}
		None
	}
}

impl Crane {
	fn parse_stacks(stack_input: Vec<&str>) -> Vec<String> {
		let len: usize = stack_input[0].len();
		let size = len/4 + (len%4 != 0) as usize;
		(0..size)
			.map(|i| stack_input
				.iter()
				.map(|line| line.chars().nth(i * 4 + 1).unwrap())
				.collect::<String>()
				.replace(' ', "")
			)
			.collect_vec()
	}

	fn parse_operations(ops_input: Vec<&str>) -> VecDeque<Operation> {
		let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

		let ops = ops_input.iter()
			.filter(|l| !l.is_empty())
			.map(|c| re.captures(c).unwrap())
			.map(|c| Operation {
				quantity: c[1].parse().unwrap(),
				source: c[2].parse().unwrap(),
				target: c[3].parse().unwrap()
			}).collect_vec();
		VecDeque::from(ops)
	}

	pub fn from_str(input: &str) -> Crane {
		let lines: Vec<&str> = input.split('\n').collect();
		let (split_index, _) = lines.iter().find_position(|l| **l == "").unwrap();


		Crane {
			stacks: Crane::parse_stacks(lines[0 .. split_index-1].to_vec()),
			operations: Crane::parse_operations(lines[split_index+1 ..].to_vec()),
		}
	}

	fn operate(&mut self) {
		
		let operation = self.operations.pop_front().unwrap();
		let split: String = self.stacks[operation.source - 1].take(operation.quantity).unwrap();
		self.stacks[operation.target - 1].insert_str(0, &split);
		println!("{:#?}", operation);
		println!("{:#?}", self.stacks);
	}
	
	pub fn operate_all(&mut self) -> String {
		(0..self.operations.len()).for_each(|_| self.operate());
		self.stacks.iter().map(|s| s.chars().next().unwrap()).collect()
	}
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

use super::{Crane, Operation};

	#[test]
	fn it_should_parse_the_input() {
		let input = "    [D]    
		[N] [C]    
		[Z] [M] [P]
		 1   2   3 
		
		move 1 from 2 to 1
		move 3 from 1 to 3
		move 2 from 2 to 1
		move 1 from 1 to 2".replace('\t', "");


		let expected = Crane {
			stacks: vec![
				String::from("NZ"),
				String::from("DCM"),
				String::from("P"),
			],
			operations: VecDeque::from_iter([
				Operation { quantity: 1, target: 1, source: 2 },
				Operation { quantity: 3, target: 3, source: 1 },
				Operation { quantity: 2, target: 1, source: 2 },
				Operation { quantity: 1, target: 2, source: 1 }
			]),
		};
		let actual = Crane::from_str(&input);
		assert_eq!(expected, actual);
	}

	#[test]
	fn it_should_perform_an_operation() {
		let mut crane = Crane {
			stacks: vec![
				String::from("NZ"),
				String::from("DCM"),
				String::from("P"),
			],
			operations: VecDeque::from_iter([
				Operation { quantity: 1, target: 1, source: 2 },
				Operation { quantity: 3, target: 3, source: 1 },
				Operation { quantity: 2, target: 1, source: 2 },
				Operation { quantity: 1, target: 2, source: 1 }
			]),
		};
		let expected =  Crane {
			stacks: vec![
				String::from("DNZ"),
				String::from("CM"),
				String::from("P"),
			],
			operations: VecDeque::from_iter([
				Operation { quantity: 3, target: 3, source: 1 },
				Operation { quantity: 2, target: 1, source: 2 },
				Operation { quantity: 1, target: 2, source: 1 }
			]),
		};
		crane.operate();
		assert_eq!(expected, crane);
	}

	#[test]
	fn it_should_perform_all_operations() {
		let mut crane = Crane {
			stacks: vec![
				String::from("NZ"),
				String::from("DCM"),
				String::from("P"),
			],
			operations: VecDeque::from_iter([
				Operation { quantity: 1, target: 1, source: 2 },
				Operation { quantity: 3, target: 3, source: 1 },
				Operation { quantity: 2, target: 1, source: 2 },
				Operation { quantity: 1, target: 2, source: 1 }
			]),
		};
		let expected = Crane {
			stacks: vec![
				String::from("M"),
				String::from("C"),
				String::from("DNZP"),
			],
			operations: VecDeque::new(),
		};
		let output = crane.operate_all();
		assert_eq!(expected, crane);
		assert_eq!("MCD", output);
	}
}