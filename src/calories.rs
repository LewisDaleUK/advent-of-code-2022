#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pack {
    food: Vec<i32>,
    total: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Calories {
    packs: Vec<Pack>,
}

impl Calories {
    pub fn from_str(input: &str) -> Calories {
        let mut packs: Vec<Pack> = vec![];
        let mut pack = Pack {
            food: vec![],
            total: 0,
        };

        for line in input.split('\n') {
            if let Ok(cals) = line.parse::<i32>() {
                pack.food.push(cals);
                pack.total += cals;
            } else {
                packs.push(pack);
                pack = Pack {
                    food: vec![],
                    total: 0,
                };
            }
        }
        if !pack.food.is_empty() {
            packs.push(pack);
        }

        Calories { packs }
    }

    fn sort(&mut self) {
        self.packs.sort_by_key(|t| t.total);
        self.packs.reverse();
    }

    pub fn max(&mut self) -> i32 {
        self.sort();
        self.packs.first().unwrap().total
    }

    pub fn total_n(&mut self, n: i32) -> i32 {
        self.sort();
        let top = self.packs.iter().take(n.try_into().unwrap());
        top.fold(0, |acc, pack| acc + pack.total)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_reads_inputs_to_product_a_list_of_packs_with_totals() {
        let input = "1000
		2000
		3000
		
		4000
		
		5000
		6000
		
		7000
		8000
		9000
		
		10000"
            .replace('\t', "");

        let calories = super::Calories::from_str(&input);
        let expected = super::Calories {
            packs: vec![
                super::Pack {
                    food: vec![1000, 2000, 3000],
                    total: 6000,
                },
                super::Pack {
                    food: vec![4000],
                    total: 4000,
                },
                super::Pack {
                    food: vec![5000, 6000],
                    total: 11000,
                },
                super::Pack {
                    food: vec![7000, 8000, 9000],
                    total: 24000,
                },
                super::Pack {
                    food: vec![10000],
                    total: 10000,
                },
            ],
        };

        assert_eq!(expected, calories);
    }
    #[test]
    fn returns_index_of_elf_with_most_calories() {
        let input = "1000
		2000
		3000
		
		4000
		
		5000
		6000
		
		7000
		8000
		9000
		
		10000"
            .replace('\t', "");

        let mut calories = super::Calories::from_str(&input);
        let result = calories.max();
        println!("{:#?}", calories);

        assert_eq!(result, 24000);
    }

    #[test]
    fn it_gets_the_calories_for_the_top_n_elves() {
        let input = "1000
		2000
		3000
		
		4000
		
		5000
		6000
		
		7000
		8000
		9000
		
		10000"
            .replace('\t', "");
        let mut calories = super::Calories::from_str(&input);
        let result = calories.total_n(3);

        assert_eq!(result, 45000);
    }
}
