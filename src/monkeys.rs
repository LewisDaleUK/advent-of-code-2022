use nom::{IResult, sequence::{tuple, delimited, preceded}, character::complete::{space0, i32 as cci32, space1, one_of}, bytes::complete::tag, multi::{many0, separated_list1}, branch::alt, combinator::{value, map}};

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add(OperationTarget),
    Divide(OperationTarget),
    Multiply(OperationTarget)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperationTarget {
    Value(i32),
    Old,
}

#[derive(Debug, PartialEq, Eq)]
pub enum If {
    Divisible(i32)
}

// DivisibleBy, If True throw to, If False throw to
type Test = (If, i32, i32);

fn parse_items(i: &str) -> IResult<&str, Vec<i32>> {
    let (i, (_, _, items,_)) = tuple(
        (space1,
            tag("Starting items: "),
            separated_list1(tag(", "), cci32),
            tag("\n")))(i)?;
    Ok((i, items))
}

fn parse_target(i: &str) -> IResult<&str, OperationTarget> {
    alt((value(OperationTarget::Old, tag("old")), map(cci32, OperationTarget::Value)))(i)
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let op_parser = preceded(tag("new = old "), tuple((one_of("*+"), space0, parse_target)));

    let (i, (_, _, (op, _, target), _)) = tuple((space1, tag("Operation: "), op_parser, tag("\n")))(i)?;

    let op = match op {
        '+' => Operation::Add(target),
        '*' => Operation::Multiply(target),
        _ => unreachable!()
    };

    Ok((i, op))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((space0, tag("Monkey "), cci32, tag(":\n")))(input)?;
    let (i, items) = parse_items(i)?;
    let (i, operation) = parse_operation(i)?;

    Ok((input, Monkey {
        items,
        operation,
        test: (If::Divisible(0), 0, 0)
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_monkey() {
        let expected = super::Monkey {
            items: vec![79, 98],
            operation: super::Operation::Multiply(super::OperationTarget::Value(19)),
            test: (super::If::Divisible(23), 2, 3)
        };

        let input = String::from("Monkey 0:\n")
         + "    Starting items: 79, 98\n"
         + "    Operation: new = old * 19\n"
         + "    Test: divisible by 23\n"
         + "        If true: throw to monkey 2\n"
         + "        If false: throw to monkey 3\n";
        
        // println!("{}", input);

        let (_, result) = super::parse_monkey(&input).unwrap();
        assert_eq!(expected, result);
    }
}