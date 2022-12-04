use std::collections::HashSet;

use itertools::Itertools;

fn parse_range(range: &str) -> HashSet<i32> {
	let pieces: Vec<i32> = range.split('-').map(|r| r.parse::<i32>().unwrap()).collect();
 	(pieces[0]..=pieces[1]).collect()
}

pub fn range_overlaps(range: &str) -> bool {
	let (a, b) = range.split(',')
		.map(parse_range)
		.collect_tuple()
		.unwrap();

	!a.is_disjoint(&b)
}

#[cfg(test)]
mod tests {
	
	#[test]
	fn it_returns_true_if_one_range_contains_another() {
		let inputs = [
			"2-4,6-8",
			"2-3,4-5",
			"5-7,7-9",
			"2-8,3-7",
			"6-6,4-6",
			"2-6,4-8",
			"50-50,1-49",
		];
		let expectations = [
			false,
			false,
			true,
			true,
			true,
			true,
			false,
		];
		assert_eq!(expectations, inputs.map(super::range_overlaps));
	}
}