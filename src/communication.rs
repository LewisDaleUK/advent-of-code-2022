use std::collections::HashSet;

pub trait FindUnique {
	fn find_unique(&self, offset: usize) -> Option<usize>;
}

impl FindUnique for str {
	fn find_unique(&self, offset: usize) -> Option<usize> {
		for idx in offset..self.len() {
			let len = HashSet::<char>::from_iter(self[idx - offset..idx].chars()).len();
			if len == offset {
				return Some(idx);
			}
		}
		None
	}
}

#[cfg(test)]
mod tests {
    use crate::communication::FindUnique;

    #[test]
    fn it_should_find_a_start_of_packet_marker_index() {
        let inputs = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let expectations: [Option<usize>; 5] = [Some(7), Some(5), Some(6), Some(10), Some(11)];
        assert_eq!(expectations, inputs.map(|i| i.find_unique(4)));
    }

    #[test]
    fn it_should_find_a_start_of_message_marker_index() {
        let inputs = [
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let expectations: [Option<usize>; 5] = [Some(19), Some(23), Some(23), Some(29), Some(26)];
        assert_eq!(expectations, inputs.map(|i| i.find_unique(14)));
    }
}
