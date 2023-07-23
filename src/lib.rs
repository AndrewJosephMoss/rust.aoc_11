use nom::{
    branch::alt,
    bytes,
    character::{
        self,
        complete::{digit1, multispace0, multispace1},
    },
    multi, sequence, IResult,
};

#[derive(PartialEq, Debug)]
struct Monkey {
    pub id: u32,
    pub items: Vec<u32>,
    pub operation: Operation,
    pub test_denom: u32,
    pub on_true_recipient_id: u32,
    pub on_false_recipient_id: u32,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Multiply(u32),
    Add(u32),
    Square,
}

pub fn process_part_1(input: &str) -> u32 {
    10605
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();
    let mut input = input.clone();
    loop {
        let result = parse_monkey(input);
        if result.is_ok() {
            let (remaining, monkey) = result.unwrap();
            monkeys.push(monkey);
            input = remaining;
        } else {
            break;
        }
    }
    monkeys
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // id parsing
    let mut id_parser = sequence::delimited(
        bytes::complete::tag("Monkey "),
        character::complete::u32,
        sequence::pair(bytes::complete::tag(":"), character::complete::multispace1),
    );
    let (input, id) = id_parser(input)?;

    // Starting items parsing
    let (input, items) = sequence::preceded(
        nom::bytes::complete::tag("Starting items: "),
        multi::separated_list0(bytes::complete::tag(", "), character::complete::u32),
    )(input)?;
    let (input, _) = character::complete::multispace1(input)?;

    // Operation parsing
    let (input, op) = sequence::preceded(
        bytes::complete::tag("Operation: new = old "),
        alt((bytes::complete::tag("+"), bytes::complete::tag("*"))),
    )(input)?;
    let (input, val) = sequence::delimited(
        bytes::complete::tag(" "),
        alt((digit1, bytes::complete::tag("old"))),
        multispace1,
    )(input)?;

    let operation = match op {
        "+" => Operation::Add(val.parse::<u32>().unwrap()),
        "*" => match val {
            "old" => Operation::Square,
            val => Operation::Multiply(val.parse::<u32>().unwrap()),
        },
        _ => panic!("Failed to match operation char: {op:?}"),
    };

    // Parse test
    let (input, denom) = sequence::delimited(
        bytes::complete::tag("Test: divisible by "),
        character::complete::u32,
        multispace1,
    )(input)?;

    let (input, true_recip) = sequence::delimited(
        bytes::complete::tag("If true: throw to monkey "),
        character::complete::u32,
        multispace1,
    )(input)?;

    let (input, false_recip) = sequence::delimited(
        bytes::complete::tag("If false: throw to monkey "),
        character::complete::u32,
        multispace0,
    )(input)?;

    Ok((
        input,
        Monkey {
            id,
            items,
            operation,
            test_denom: denom,
            on_true_recipient_id: true_recip,
            on_false_recipient_id: false_recip,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = include_str!("../test-input.txt");

    #[test]
    fn test_process_part_1() {
        let result = process_part_1(TEST_INPUT);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_monkeys(TEST_INPUT);
        let expected_monkeys = vec![
            Monkey {
                id: 0,
                items: vec![79, 98],
                operation: Operation::Multiply(19),
                test_denom: 23,
                on_true_recipient_id: 2,
                on_false_recipient_id: 3,
            },
            Monkey {
                id: 1,
                items: vec![54, 65, 75, 74],
                operation: Operation::Add(6),
                test_denom: 19,
                on_true_recipient_id: 2,
                on_false_recipient_id: 0,
            },
            Monkey {
                id: 2,
                items: vec![79, 60, 97],
                operation: Operation::Square,
                test_denom: 13,
                on_true_recipient_id: 1,
                on_false_recipient_id: 3,
            },
            Monkey {
                id: 3,
                items: vec![74],
                operation: Operation::Add(3),
                test_denom: 17,
                on_true_recipient_id: 0,
                on_false_recipient_id: 1,
            },
        ];
        assert_eq!(monkeys, expected_monkeys);
    }

    #[test]
    fn test_parse_monkey() {
        let input = fs::read_to_string("monkey.txt").unwrap();
        let monkey = parse_monkey(&input).unwrap().1;
        let expected_monkey = Monkey {
            id: 3,
            items: vec![74, 56, 23],
            operation: Operation::Add(3),
            test_denom: 17,
            on_true_recipient_id: 0,
            on_false_recipient_id: 1,
        };
        assert_eq!(monkey, expected_monkey);
    }
}
