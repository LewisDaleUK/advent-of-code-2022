use num_integer::Roots;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    U,
    D,
    L,
    R,
}

impl Move {
    pub fn from_str(cmd: &str) -> Option<Move> {
        match cmd {
            "D" => Some(Move::D),
            "L" => Some(Move::L),
            "U" => Some(Move::U),
            "R" => Some(Move::R),
            _ => None,
        }
    }
}

type Position = (isize, isize);

trait Vector {
    fn normalize(&mut self) -> &Self;
    fn sub(&mut self, other: Self) -> &Self;
    fn magnitude(&self) -> isize;
    fn divide(&mut self, by: isize) -> &Self;
    fn distance(&self, other: Self) -> usize;
    fn add(&mut self, other: Self) -> &Self;
    fn adjacent(&self, other: Self) -> bool;
}

impl Vector for Position {
    fn normalize(&mut self) -> &Self {
        self.divide(self.magnitude())
    }

    fn sub(&mut self, other: Self) -> &Self {
        self.0 -= other.0;
        self.1 -= other.1;
        self
    }

    fn magnitude(&self) -> isize {
        (self.0.pow(2) + self.1.pow(2)).sqrt()
    }

    fn divide(&mut self, by: isize) -> &Self {
        self.0 = if self.0 < 0 {
            num_integer::Integer::div_floor(&self.0, &by)
        } else {
            num_integer::Integer::div_ceil(&self.0, &by)
        };

        self.1 = if self.1 < 0 {
            num_integer::Integer::div_floor(&self.1, &by)
        } else {
            num_integer::Integer::div_ceil(&self.1, &by)
        };
        self
    }

    fn distance(&self, other: Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn add(&mut self, other: Self) -> &Self {
        self.0 += other.0;
        self.1 += other.1;
        self
    }

    fn adjacent(&self, other: Self) -> bool {
        let mut diff = *self;
        diff.sub(other);
        diff.magnitude() <= 1
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Knot {
    position: Position,
    history: HashSet<Position>,
}

impl Default for Knot {
    fn default() -> Self {
        Knot {
            position: (0, 0),
            history: HashSet::from_iter(vec![(0, 0)]),
        }
    }
}

impl Knot {
    pub fn visited(&self) -> usize {
        self.history.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RopeBridge {
    pub knots: Vec<Knot>,
}

impl Default for RopeBridge {
    fn default() -> RopeBridge {
        RopeBridge {
            knots: vec![Knot::default(), Knot::default()],
        }
    }
}

impl RopeBridge {
    pub fn new(count: usize) -> RopeBridge {
        RopeBridge {
            knots: (0..count).map(|_| Knot::default()).collect(),
        }
    }

    pub fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }

    fn align_tail(&mut self) {
        for i in 1..self.knots.len() {
            let mut head = self.knots.get(i - 1).unwrap().position;
            let mut tail = self.knots.get(i).unwrap().position;

            if !head.adjacent(tail) {
                head.sub(tail);
                head.normalize();
                tail.add(head);

                let mut last = self.knots.get_mut(i).unwrap();
                last.position = tail;
                last.history.insert(tail);
            }
        }
    }

    pub fn perform_move(&mut self, movement: Move, amount: usize) {
        for _ in 0..amount {
            let mut head = self.knots.first_mut().unwrap();
            match movement {
                Move::D => head.position.1 -= 1,
                Move::U => head.position.1 += 1,
                Move::L => head.position.0 -= 1,
                Move::R => head.position.0 += 1,
            };
            head.history.insert(head.position);

            self.align_tail();
        }
    }

    pub fn process(&mut self, input: Vec<&str>) {
        for line in input {
            let (action, amount) = line.split_once(' ').unwrap();
            let action = Move::from_str(action);
            self.perform_move(action.unwrap(), amount.parse().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rope::Move;
    use std::collections::HashSet;

    use super::{Knot, RopeBridge};

    #[test]
    fn it_should_create_a_default_rope_bridge() {
        let expected = RopeBridge {
            knots: vec![
                Knot {
                    position: (0, 0),
                    history: HashSet::from_iter(vec![(0, 0)]),
                },
                Knot {
                    position: (0, 0),
                    history: HashSet::from_iter(vec![(0, 0)]),
                },
            ],
        };
        let actual = RopeBridge::default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_perform_a_move() {
        let mut bridge = RopeBridge::default();
        bridge.perform_move(Move::R, 4);
        bridge.perform_move(Move::U, 4);
        bridge.perform_move(Move::L, 3);
        bridge.perform_move(Move::D, 1);
        bridge.perform_move(Move::R, 4);
        bridge.perform_move(Move::D, 1);
        bridge.perform_move(Move::L, 5);
        bridge.perform_move(Move::R, 2);

        let expected = HashSet::from_iter(vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (3, 4),
            (2, 4),
            (3, 3),
            (4, 3),
            (3, 2),
            (2, 2),
            (1, 2),
        ]);

        let expected_head = HashSet::from_iter(vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
            (3, 4),
            (2, 4),
            (1, 4),
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
            (5, 3),
            (5, 2),
            (4, 2),
            (3, 2),
            (2, 2),
            (1, 2),
            (0, 2),
            (1, 2),
            (2, 2),
        ]);
        assert_eq!(bridge.knots.first().unwrap().history, expected_head);
        assert_eq!(expected, bridge.tail().history);
        assert_eq!(bridge.tail().visited(), 13);
    }

    #[test]
    pub fn it_simulates_a_larger_rope() {
        let input = "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"
            .replace('\t', "")
            .replace("    ", "");
        let mut bridge = RopeBridge::new(10);

        bridge.process(input.lines().collect());
        assert_eq!(bridge.tail().visited(), 36);
    }
}
