fn main() {
    fizz_buzz("fizz", "buzz", 3, 5)
}

fn fizz_buzz(fizz_word: &str, buzz_word: &str, fizz_n: i32, buzz_n: i32) {
    for i in 1..100 {
        match (i% fizz_n, i% buzz_n) {
            (0, 0) => println!("{0}{1}", fizz_word, buzz_word),
            (0, _) => println!("{}", fizz_word),
            (_, 0) => println!("{}", buzz_word),
            (_, _) => println!("{}", i)
        }
    }
}
