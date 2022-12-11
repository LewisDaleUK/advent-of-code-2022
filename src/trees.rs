use grid::{grid, Grid};
use itertools::Itertools;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LineOfSight<T> {
    north: Vec<T>,
    south: Vec<T>,
    east: Vec<T>,
    west: Vec<T>,
}

trait Normalize {
    fn normalize(&self) -> usize;
}

impl Normalize for isize {
    fn normalize(&self) -> usize {
        if self.is_negative() {
            0
        } else {
            self.unsigned_abs()
        }
    }
}

pub trait HasLineOfSight<T> {
    fn lines_of_sight(&self, u_row: usize, u_col: usize) -> LineOfSight<T>;
}

impl HasLineOfSight<i32> for Grid<i32> {
    fn lines_of_sight(&self, u_row: usize, u_col: usize) -> LineOfSight<i32> {
        let col = self.iter_col(u_col);
        let row = self.iter_row(u_row);

        LineOfSight {
            north: col.clone().collect_vec()[..u_row]
                .iter()
                .map(|i| **i)
                .rev()
                .collect_vec(),
            south: col.clone().collect_vec()[u_row + 1..]
                .iter()
                .map(|i| **i)
                .collect_vec(),
            east: row.clone().collect_vec()[u_col + 1..]
                .iter()
                .map(|i| **i)
                .collect_vec(),
            west: row.clone().collect_vec()[..u_col]
                .iter()
                .map(|i| **i)
                .rev()
                .collect_vec(),
        }
    }
}

pub trait Visible<A, T: HasLineOfSight<A> = Self> {
    fn visible(&self, row: usize, col: usize) -> bool;
    fn count_visible(&self) -> usize;
}

trait HasVisible<A> {
    fn has_visible(&self, cmp: A) -> bool;
    fn scene_score(&self, cmp: A) -> usize;
}

impl HasVisible<i32> for Vec<i32> {
    fn has_visible(&self, cmp: i32) -> bool {
        self.iter().copied().filter(|v| *v >= cmp).count() == 0
    }

    fn scene_score(&self, cmp: i32) -> usize {
        let mut score = 0;
        for entry in self.iter() {
            score += 1;
            if *entry >= cmp {
                break;
            }
        }

        score
    }
}

impl Visible<i32> for Grid<i32> {
    fn visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row == self.rows() - 1 || col == self.cols() - 1 {
            true
        } else {
            let value = self.get(row, col).unwrap();
            let los = self.lines_of_sight(row, col);

            los.north.has_visible(*value)
                || los.south.has_visible(*value)
                || los.east.has_visible(*value)
                || los.west.has_visible(*value)
        }
    }

    fn count_visible(&self) -> usize {
        let mut count = 0;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                if self.visible(row, col) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub trait Scenic {
    fn scene_score(&self, row: usize, col: usize) -> usize;
    fn highest_score(&self) -> usize;
}

impl Scenic for Grid<i32> {
    fn scene_score(&self, row: usize, col: usize) -> usize {
        if row == 0 || col == 0 || row == self.rows() - 1 || col == self.cols() - 1 {
            0
        } else {
            let item = *self.get(row, col).unwrap();
            let los = self.lines_of_sight(row, col);
            los.north.scene_score(item)
                * los.south.scene_score(item)
                * los.east.scene_score(item)
                * los.west.scene_score(item)
        }
    }

    fn highest_score(&self) -> usize {
        let mut high_score = 0;
        for row in 0..self.rows() {
            for col in 0..self.rows() {
                let score = self.scene_score(row, col);
                if score > high_score {
                    high_score = score;
                }
            }
        }
        high_score
    }
}

pub fn construct(inputs: Vec<&str>) -> Grid<i32> {
    let mut grid: Grid<i32> = grid![];
    for line in inputs.iter() {
        let row: Vec<i32> = line
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect();
        grid.push_row(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use grid::grid;
    use itertools::Itertools;

    use super::{construct, HasLineOfSight, LineOfSight, Scenic, Visible};

    #[test]
    fn it_constructs_a_grid() {
        let binding = "30373
		25512
		65332
		33549
		35390"
            .replace('\t', "");
        let input = binding.split('\n').collect_vec();

        let expected = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];
        let result = construct(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_gets_neighbours_for_an_entry() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];
        let expected = LineOfSight {
            north: vec![5, 3],
            south: vec![5, 3],
            east: vec![3, 2],
            west: vec![5, 6],
        };
        let result = grid.lines_of_sight(2, 2);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_gets_neighbours_for_an_entry_at_edge() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];
        let expected = LineOfSight {
            north: vec![9, 2, 2, 3],
            south: vec![],
            east: vec![],
            west: vec![9, 3, 5, 3],
        };
        let result = grid.lines_of_sight(4, 4);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_tests_entry_for_visibility() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];
        assert!(grid.visible(1, 1));
        assert_eq!(grid.visible(2, 2), false);
        assert!(grid.visible(2, 3));
        assert_eq!(grid.visible(3, 1), false);
        assert!(grid.visible(3, 2))
    }

    #[test]
    fn it_tests_for_visiblity() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];

        let inputs = (1..grid.rows() - 1)
            .flat_map(|row| (1..grid.cols() - 1).map(|col| (row, col)).collect_vec())
            .collect_vec();

        let expected: Vec<bool> = [
            [true, true, false],
            [true, false, true],
            [false, true, false],
        ]
        .iter()
        .flatten()
        .copied()
        .collect_vec();

        let result = inputs
            .iter()
            .map(|(row, col)| grid.visible(*row, *col))
            .collect_vec();
        assert_eq!(expected, result);
    }

    #[test]
    fn it_counts_the_visible_trees() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];
        let result = grid.count_visible();
        assert_eq!(result, 21);
    }

    #[test]
    fn it_counts_the_scene_score() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];

        assert_eq!(grid.scene_score(0, 0), 0);
        assert_eq!(grid.scene_score(1, 2), 4);
        assert_eq!(grid.scene_score(3, 2), 8);
    }

    #[test]
    fn it_finds_the_highest_score() {
        let grid = grid![
            [3,0,3,7,3]
            [2,5,5,1,2]
            [6,5,3,3,2]
            [3,3,5,4,9]
            [3,5,3,9,0]
        ];
        assert_eq!(8, grid.highest_score());
    }
}
