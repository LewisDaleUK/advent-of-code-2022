#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Moves {
	Rock,
	Paper,
	Scissors
} 

#[derive(Debug, PartialEq, Eq)]
pub struct Round {
	pub first: Moves,
	pub second: Moves
}

impl Round {
	pub fn from_cols(column_a: &str, column_b: &str) -> Round {
		let first = match column_a {
			"A" => Moves::Rock,
			"B" => Moves::Paper,
			"C" => Moves::Scissors,
			_ => Moves::Rock,
		};
		let second = match column_b {
			"X" => match first {
				Moves::Rock => Moves::Scissors,
				Moves::Paper => Moves::Rock,
				Moves::Scissors => Moves::Paper
			}
			"Y" => first,
			"Z" => match first {
				Moves::Rock => Moves::Paper,
				Moves::Paper => Moves::Scissors,
				Moves::Scissors => Moves::Rock
			},
			_ => Moves::Rock
		};
		Round { first, second }
	}

	fn shape_score(&self, player_move: &Moves) -> i32 {
		match player_move {
			Moves::Rock => 1,
			Moves::Paper => 2,
			Moves::Scissors => 3, 
		}
	}

	fn round_score(&self) -> i32 {
		match self.first {
			Moves::Rock => {
				match self.second {
					Moves::Rock => 3,
					Moves::Paper => 6,
					Moves::Scissors => 0,
				}
			},
			Moves::Paper => {
				match self.second {
					Moves::Rock => 0,
					Moves::Paper => 3,
					Moves::Scissors => 6,
				}
			},
			Moves::Scissors => {
				match self.second {
					Moves::Rock => 6,
					Moves::Paper => 0,
					Moves::Scissors => 3
				}
			}
		}
	}

	pub fn score(&self) -> i32 {
		let shape_score = self.shape_score(&self.second);
		let game_score = self.round_score();
		

		shape_score + game_score
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
	rounds: Vec<Round>,
}

impl Game {
	pub fn from_str(input: &str) -> Game {
		let mut rounds: Vec<Round> = vec![];
		for line in input.split('\n').into_iter() {
			if !line.is_empty() {
				let cols: Vec<&str> = line.split(' ').take(2).collect();
				rounds.push(Round::from_cols(cols[0], cols[1]));
			}	
		}

		Game { rounds }
	}

	pub fn total_score(&self) -> i32 {
		self.rounds.iter().map(|r| r.score()).sum()
	}
}

#[cfg(test)]
mod tests {
	use super::{Game, Moves, Round};

	#[test]
	fn it_can_construct_a_round_from_cols() {
		let expected = Round {
			first: Moves::Rock,
			second: Moves::Paper
		};

		let actual = Round::from_cols("A", "Z");
		assert_eq!(actual, expected);
	}

	#[test]
	fn it_should_compute_the_score() {
		let round = Round::from_cols("A", "Y");
		assert_eq!(round.score(), 4);
	}

	#[test]
	fn it_should_parse_a_full_game() {
		let expected = Game {
			rounds: vec![
				Round { first: Moves::Rock, second: Moves::Rock },
				Round { first: Moves::Paper, second: Moves::Rock },
				Round { first: Moves::Scissors, second: Moves::Rock }
			]
		};
		let input = "A Y
		B X
		C Z".replace('\t', "");
		
		let actual = Game::from_str(&input);
		assert_eq!(expected, actual);
	}

	#[test]
	fn it_calculates_the_total_score() {
		let game = Game {
			rounds: vec![
				Round { first: Moves::Rock, second: Moves::Paper },
				Round { first: Moves::Paper, second: Moves::Rock },
				Round { first: Moves::Scissors, second: Moves::Scissors }
			]
		};
		let score = game.total_score();
		assert_eq!(score, 15);
	}
}