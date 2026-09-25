fn fizzbuzz(n: u32) -> String {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => "FizzBuzz".to_owned(),
        (true, false) => "Fizz".to_owned(),
        (false, true) => "Buzz".to_owned(),
        (false, false) => n.to_string(),
    }
}

fn main() {
    for n in 1..=100 {
        println!("{}", fizzbuzz(n));
    }
}

#[cfg(test)]
mod tests {
    use super::fizzbuzz;

    #[test]
    fn labels_numbers_by_divisibility() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }
}
