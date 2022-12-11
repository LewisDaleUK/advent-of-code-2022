use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct CPU {
    x: i32,
    pub cycles: Vec<i32>,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            x: 1,
            cycles: vec![1]
        }
    }
}

impl CPU {
    fn cycle(&mut self) {
        self.cycles.push(self.x);
        let cycle = self.cycles.len() as i32;

        if [self.x, self.x + 1, self.x + 2].contains(&(cycle % 40)) {
            print!("#");
        } else {
            print!(".");
        }

        if cycle % 40 == 0 {
            print!("\n");
        }
    }
    pub fn execute(&mut self, ins: Instruction) {
        self.cycle();
        match ins {
            Instruction::Addx(count) => {
                self.x += count;
                self.cycle();
            },
            Instruction::Noop => ()
        };
    }

    pub fn run_program(&mut self, lines: Vec<&str>) {
        for line in lines {
            let parts = line.split(' ').collect_vec();
            let instruction = match parts[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(parts[1].parse().unwrap()),
                _ => panic!("Instruction not recognised")
            };
            self.execute(instruction);
        }
    }

    pub fn signal_strength_at(&self, idx: usize) -> i32 {
        self.cycles.get(idx - 1).unwrap() * idx as i32
    }

    pub fn total_signal_strength(&self) -> i32 {
        [20, 60, 100, 140, 180, 220].iter().map(|idx| self.signal_strength_at(*idx as usize))
            .fold(0, |acc, strength| acc + strength)
    }
}

pub enum Instruction {
    Noop,
    Addx(i32)
}

#[cfg(test)]
mod tests {
    use super::{CPU, Instruction};

    #[test]
    fn it_creates_a_new_cpu() {
        let cpu = CPU::default();
        let expected = CPU {
            x: 1,
            cycles: vec![1]
        };
        assert_eq!(expected, cpu);
    }

    #[test]
    fn it_handles_addx() {
        let mut cpu = CPU::default();
        cpu.execute(Instruction::Addx(10));

        let expected = CPU {
            x: 11,
            cycles: vec![1, 1, 11],
        };
        assert_eq!(expected, cpu);
    }

    #[test]
    fn it_handles_noop() {
        let mut cpu = CPU::default();
        cpu.execute(Instruction::Noop);
        let expected = CPU {
            x: 1,
            cycles: vec![1, 1]
        };
        assert_eq!(expected, cpu);
    }

    #[test]
    fn it_executes_a_small_program() {
        let input = "noop
        addx 3
        addx -5".replace('\t', "").replace("    ", "");
        let lines = input.lines();

        let mut cpu = CPU::default();
        cpu.run_program(lines.collect());

        let expected = CPU {
            x: -1,
            cycles: vec![1, 1, 1, 4, 4, -1]
        };
        assert_eq!(expected, cpu);
    }

    #[test]
    fn it_handles_a_larger_file() {
        let input = "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop".replace('\t', "").replace("    ", "");
        let lines = input.lines();

        let mut cpu = CPU::default();
        cpu.run_program(lines.collect());

        assert_eq!(420, cpu.signal_strength_at(20));
        assert_eq!(1140, cpu.signal_strength_at(60));
        assert_eq!(1800, cpu.signal_strength_at(100));
        assert_eq!(2940, cpu.signal_strength_at(140));
        assert_eq!(2880, cpu.signal_strength_at(180));
        assert_eq!(3960, cpu.signal_strength_at(220));
        assert_eq!(13140, cpu.total_signal_strength());
    }
}