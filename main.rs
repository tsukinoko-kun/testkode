fn main() {
    for number in 1..=100 {
        match (number % 3 == 0, number % 5 == 0) {
            (true, true) => println!("Fizzbuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("-> {number}"),
        }
    }
}
