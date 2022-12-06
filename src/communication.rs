use std::collections::HashSet;

pub fn find_marker_index(input: &str, marker_offset: usize) -> Option<usize> {
    let mut set: HashSet<char>;

    for idx in marker_offset..input.len() {
        set = HashSet::from_iter(input[idx - marker_offset..idx].chars());
        if set.len() == marker_offset {
            return Some(idx);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::find_marker_index;

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
        assert_eq!(expectations, inputs.map(|i| find_marker_index(i, 4)));
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
        assert_eq!(expectations, inputs.map(|i| find_marker_index(i, 14)));
    }

    #[test]
    fn it_should_find_a_start_of_message_marker_index_2() {
        let inputs = ["mjqjpqmgbljsphdztnvjfqwrcgsmlb"];
        let expectations: [Option<usize>; 1] = [Some(19)];
        assert_eq!(expectations, inputs.map(|i| find_marker_index(i, 14)));
    }
}
