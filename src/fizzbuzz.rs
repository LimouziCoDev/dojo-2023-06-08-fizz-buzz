use std::borrow::ToOwned;
use std::string::ToString;

struct Rule {
    multiple: u8,
    result: String,
}

impl Rule {
    fn is_multiple_of(&self, n: u8) -> bool {
        n % self.multiple == 0
    }

    fn result(&self) -> String {
        self.result.to_owned()
    }
}


fn print_it(n: u8) -> String {
    let mut result = vec![
        Rule { multiple: 3, result: "Fizz".to_owned() },
        Rule { multiple: 5, result: "Buzz".to_owned() },
    ].iter()
        .filter(|rule| rule.is_multiple_of(n))
        .map(|rule| rule.result())
        .collect::<String>();
    if result.is_empty() {
        result = n.to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_print_1_when_given_1() {
        let result = print_it(1);
        assert_eq!(result, "1");
    }

    #[test]
    fn should_print_2_when_given_2() {
        let result = print_it(2);
        assert_eq!(result, "2");
    }

    #[test]
    fn should_print_fizz_when_given_3() {
        let result = print_it(3);
        assert_eq!(result, "Fizz");
    }

    #[test]
    fn should_print_buzz_when_given_5() {
        let result = print_it(5);
        assert_eq!(result, "Buzz");
    }

    #[test]
    fn should_print_fizz_when_given_6() {
        let result = print_it(6);
        assert_eq!(result, "Fizz");
    }

    #[test]
    fn should_print_buzz_when_given_10() {
        let result = print_it(10);
        assert_eq!(result, "Buzz");
    }

    #[test]
    fn should_print_fizzbuzz_when_given_15() {
        let result = print_it(15);
        assert_eq!(result, "FizzBuzz");
    }

    #[test]
    fn should_print_fizzbuzz_when_given_30() {
        let result = print_it(30);
        assert_eq!(result, "FizzBuzz");
    }
}
