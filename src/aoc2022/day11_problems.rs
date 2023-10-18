mod day11 {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1, line_ending, multispace1, space1},
        combinator::{map, map_res},
        multi::{many1, separated_list0},
        sequence::{preceded, terminated, tuple},
        IResult,
    };
    use quickcheck::{Arbitrary, Gen};
    use std::fmt;

    pub trait InputParsable {
        fn parse(input: &str) -> IResult<&str, Self>
        where
            Self: Sized;
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct MonkeyNumber(pub u64);

    #[derive(Debug, PartialEq, Clone)]
    pub struct StartingItems(pub Vec<u64>);

    #[derive(Debug, PartialEq, Clone)]
    pub enum Operator {
        Plus(u64),
        Multiply(u64),
    }
    impl Arbitrary for Monkey {
        fn arbitrary(g: &mut Gen) -> Monkey {
            Monkey {
                number: MonkeyNumber(Arbitrary::arbitrary(g)),
                starting_items: Arbitrary::arbitrary(g),
                operation: Arbitrary::arbitrary(g),
                test: Arbitrary::arbitrary(g),
            }
        }
    }
    impl Arbitrary for MonkeyNumber {
        fn arbitrary(g: &mut Gen) -> MonkeyNumber {
            MonkeyNumber(Arbitrary::arbitrary(g))
        }
    }

    impl Arbitrary for StartingItems {
        fn arbitrary(g: &mut Gen) -> StartingItems {
            StartingItems::new(Arbitrary::arbitrary(g))
        }
    }

    impl Arbitrary for Operator {
        fn arbitrary(g: &mut Gen) -> Operator {
            // Assuming Operator is an enum with variants Plus and Minus
            if Arbitrary::arbitrary(g) {
                Operator::Plus(Arbitrary::arbitrary(g))
            } else {
                Operator::Multiply(Arbitrary::arbitrary(g))
            }
        }
    }

    impl Arbitrary for MonkeyTest {
        fn arbitrary(g: &mut Gen) -> MonkeyTest {
            MonkeyTest {
                test: Arbitrary::arbitrary(g),
                if_true: Arbitrary::arbitrary(g),
                if_false: Arbitrary::arbitrary(g),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct MonkeyTest {
        pub test: u64,
        pub if_true: u64,
        pub if_false: u64,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Monkey {
        pub number: MonkeyNumber,
        pub starting_items: StartingItems,
        pub operation: Operator,
        pub test: MonkeyTest,
    }
    impl fmt::Display for Monkey {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fn indent_string(input: &str, spaces: usize) -> String {
                let indent = " ".repeat(spaces);
                input
                    .lines()
                    .map(|line| format!("{}{}", indent, line))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            let starting_items = self
                .starting_items
                .items()
                .iter()
                .map(|f| (*f).to_string())
                .collect::<Vec<String>>()
                .join(", ");
            let operation = match self.operation {
                Operator::Plus(n) => format!("new = old + {}", n),
                Operator::Multiply(n) => format!("new = old * {}", n),
            };

            // Indent each line of the string.
            write!(
                f,
                "Monkey {}:\n  Starting items: {}\n  Operation: {}\n{}",
                self.number.0,
                starting_items,
                operation,
                indent_string(&self.test.to_string(), 2)
            )
        }
    }
    impl fmt::Display for MonkeyTest {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Test: divisible by {}\n  If true: throw to monkey {}\n  If false: throw to monkey {}",
                self.test, self.if_true, self.if_false)
        }
    }

    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Operator::Plus(n) => write!(f, "+ {}", n),
                Operator::Multiply(n) => write!(f, "* {}", n),
            }
        }
    }
    impl fmt::Display for StartingItems {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let items = self
                .items()
                .iter()
                .map(|f| (*f).to_string())
                .collect::<Vec<String>>()
                .join(", ");
            write!(f, "{}", items)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Monkeys(pub Vec<Monkey>);

    impl InputParsable for MonkeyNumber {
        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                preceded(tag("Monkey "), terminated(parse_number, tag(":"))),
                MonkeyNumber,
            )(input)
        }
    }

    impl InputParsable for StartingItems {
        fn parse(input: &str) -> IResult<&str, Self> {
            let (input, items) = preceded(
                tag("  Starting items: "),
                terminated(separated_list0(tag(", "), parse_number), line_ending),
            )(input)?;
            Ok((input, Self(items)))
        }
    }

    impl InputParsable for Operator {
        fn parse(input: &str) -> IResult<&str, Self> {
            let parse_plus = map(
                preceded(char('+'), preceded(space1, parse_number)),
                Operator::Plus,
            );
            let parse_multiply = map(
                preceded(char('*'), preceded(space1, parse_number)),
                Operator::Multiply,
            );

            alt((parse_plus, parse_multiply))(input)
        }
    }

    impl InputParsable for MonkeyTest {
        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                tuple((
                    preceded(
                        indent(2),
                        preceded(
                            tag("Test: divisible by "),
                            terminated(map_res(digit1, str::parse::<u64>), line_ending),
                        ),
                    ),
                    preceded(
                        indent(4),
                        preceded(
                            tag("If true: throw to monkey "),
                            terminated(map_res(digit1, str::parse::<u64>), line_ending),
                        ),
                    ),
                    preceded(
                        indent(4),
                        preceded(
                            tag("If false: throw to monkey "),
                            map_res(digit1, str::parse::<u64>),
                        ),
                    ),
                )),
                |(test, if_true, if_false)| Self {
                    test,
                    if_true,
                    if_false,
                },
            )(input)
        }
    }

    impl InputParsable for Monkey {
        fn parse(input: &str) -> IResult<&str, Self> {
            map(
                tuple((
                    terminated(MonkeyNumber::parse, line_ending),
                    StartingItems::parse,
                    parse_operation(),
                    MonkeyTest::parse,
                )),
                |(number, starting_items, operation, test)| Self {
                    number,
                    starting_items,
                    operation,
                    test,
                },
            )(input)
        }
    }

    impl InputParsable for Monkeys {
        fn parse(input: &str) -> IResult<&str, Self> {
            let (input, vec) = separated_list0(multispace1, Monkey::parse)(input)?;
            Ok((input, Self(vec)))
        }
    }

    impl StartingItems {
        pub fn new(items: Vec<u64>) -> Self {
            Self(items)
        }

        pub fn items(&self) -> &Vec<u64> {
            &self.0
        }
    }

    impl Monkeys {
        pub fn new(monkeys: Vec<Monkey>) -> Self {
            Self(monkeys)
        }

        pub fn monkeys(&self) -> &Vec<Monkey> {
            &self.0
        }
    }

    fn parse_number(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse::<u64>)(input)
    }

    fn parse_operation<'a>() -> impl Fn(&'a str) -> IResult<&str, Operator> {
        move |input| {
            preceded(
                tag("  Operation: new = old "),
                terminated(Operator::parse, line_ending),
            )(input)
        }
    }

    fn indent(level: usize) -> impl Fn(&str) -> IResult<&str, Vec<&str>> {
        move |input: &str| {
            let (input, spaces) = many1(space1)(input)?;
            let total_spaces: usize = spaces.iter().map(|s| s.len()).sum();
            if total_spaces == level {
                Ok((input, spaces))
            } else {
                let err = nom::error::Error::new(input, nom::error::ErrorKind::Verify);
                Err(nom::Err::Failure(err))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::day11::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_monkey_number_parse(num: u64) -> bool {
        let input = format!("Monkey {}: ", num);
        let result = MonkeyNumber::parse(&input);
        match result {
            Ok((_, MonkeyNumber(n))) => n == num,
            _ => false,
        }
    }

    #[quickcheck]
    fn test_starting_items_parse(items: Vec<u64>) -> bool {
        let input = format!(
            "  Starting items: {}\n",
            items
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        let result = StartingItems::parse(&input);
        match result {
            Ok((_, StartingItems(parsed_items))) => parsed_items == items,
            _ => false,
        }
    }

    #[quickcheck]
    fn test_operator_parse_plus(num: u64) -> bool {
        let input = format!("+ {}\n", num);
        let result = Operator::parse(&input);
        match result {
            Ok((_, Operator::Plus(n))) => n == num,
            _ => false,
        }
    }

    #[quickcheck]
    fn test_operator_parse_multiply(num: u64) -> bool {
        let input = format!("* {}\n", num);
        let result = Operator::parse(&input);
        println!("debug ->  input:{:?} parsed: {:?}", num, result);
        match result {
            Ok((_, Operator::Multiply(n))) => n == num,
            _ => false,
        }
    }

    #[quickcheck]
    fn test_monkey_test_parse(test: u64, if_true: u64, if_false: u64) -> bool {
        let input = format!(
            "  Test: divisible by {}\n    If true: throw to monkey {}\n    If false: throw to monkey {}\n",
            test, if_true, if_false
        );
        let result = MonkeyTest::parse(&input);
        match result {
            Ok((
                _,
                MonkeyTest {
                    test: t,
                    if_true: t_true,
                    if_false: t_false,
                },
            )) => t == test && t_true == if_true && t_false == if_false,
            _ => false,
        }
    }
    fn indent_string(input: &str, spaces: usize) -> String {
        let indent = " ".repeat(spaces);
        input
            .lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[quickcheck]
    fn test_monkey_parse(num: u64, items: Vec<u64>, op: Operator, test: MonkeyTest) -> bool {
        let input = format!(
            "Monkey {}:\n  Starting items: {}\n  Operation: new = old {}\n{}",
            num,
            StartingItems::new(items.clone()).to_string(),
            op.to_string(),
            indent_string(&test.to_string(), 2),
        );
        println!("debug ->  input:{:?}", input);
        let result = Monkey::parse(&input);
        match result {
            Ok((
                _,
                Monkey {
                    number: MonkeyNumber(n),
                    starting_items: s,
                    operation: o,
                    test: t,
                },
            )) => n == num && s.items() == &items && o == op && t == test,
            _ => false,
        }
    }

    #[quickcheck]
    fn test_monkeys_parse(monkeys: Vec<Monkey>) -> bool {
        let input = monkeys
            .iter()
            .map(|monkey| monkey.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let result = Monkeys::parse(&input);
        match result {
            Ok((_, parsed_monkeys)) => parsed_monkeys.monkeys() == &monkeys,
            _ => false,
        }
    }
}

#[cfg(test)]
mod day11_tests {
    use super::day11::{
        InputParsable, Monkey, MonkeyNumber, MonkeyTest, Monkeys, Operator, StartingItems,
    };

    const MONKEY_DATA: &str = "\
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
";

    const MONKEY_DATA_STRING_2_MONKEYS: &str = "\
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
    
Monkey 2:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 7
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

    #[test]
    fn test_parse_starting_items() {
        let input = "  Starting items: 54, 65, 75, 74\n";
        let (_rest, monkey) = StartingItems::parse(input).unwrap();

        assert_eq!(monkey, StartingItems::new(vec![54, 65, 75, 74]));

        assert_eq!(_rest, "");
    }

    #[test]
    fn test_valid_monkey_test() {
        const INPUT: &str =
                "  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0";
        let (_rest, monkey) = MonkeyTest::parse(INPUT).unwrap();

        assert_eq!(
            monkey,
            MonkeyTest {
                test: 19,
                if_true: 2,
                if_false: 0
            }
        );
    }

    #[test]
    fn test_valid_monkey2() {
        let input = MONKEY_DATA;
        let (_rest, monkey) = Monkey::parse(input).unwrap();
        assert_eq!(
            monkey,
            Monkey {
                number: MonkeyNumber(1),
                starting_items: StartingItems::new(vec![54, 65, 75, 74]),
                operation: Operator::Plus(6),
                test: MonkeyTest {
                    test: 19,
                    if_true: 2,
                    if_false: 0
                }
            }
        );
    }

    #[test]
    fn test_valid_monkeys() {
        let input = MONKEY_DATA_STRING_2_MONKEYS;
        let (_rest, monkeys) = Monkeys::parse(input).unwrap();
        let monkey = &monkeys.monkeys()[0];
        assert_eq!(
            *monkey,
            Monkey {
                number: MonkeyNumber(1),
                starting_items: StartingItems::new(vec![54, 65, 75, 74]),
                operation: Operator::Plus(6),
                test: MonkeyTest {
                    test: 19,
                    if_true: 2,
                    if_false: 0
                }
            }
        );
    }

    #[test]
    fn test_valid_monkeys_2() {
        let input = MONKEY_DATA_STRING_2_MONKEYS;
        let (_rest, monkeys) = Monkeys::parse(input).unwrap();
        let monkey = &monkeys;
        assert_eq!(
            *monkey,
            Monkeys::new(vec!(
                Monkey {
                    number: MonkeyNumber(1),
                    starting_items: StartingItems::new(vec![54, 65, 75, 74]),
                    operation: Operator::Plus(6),
                    test: MonkeyTest {
                        test: 19,
                        if_true: 2,
                        if_false: 0
                    }
                },
                Monkey {
                    number: MonkeyNumber(2),
                    starting_items: StartingItems::new(vec![54, 65, 75, 74]),
                    operation: Operator::Plus(7),
                    test: MonkeyTest {
                        test: 19,
                        if_true: 2,
                        if_false: 0
                    }
                }
            ))
        );
    }

    #[test]
    fn test_valid_monkey_eq() {
        assert_eq!(
            MonkeyTest {
                test: 19,
                if_true: 2,
                if_false: 0
            },
            MonkeyTest {
                test: 19,
                if_true: 2,
                if_false: 0
            }
        );
    }
}
