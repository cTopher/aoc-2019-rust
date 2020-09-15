use std::time::SystemTime;
use std::ops::Range;

struct Password {
    digits: Vec<u8>
}

impl Password {
    fn new(input: u32) -> Password {
        let mut rest = input;
        let mut digits = Vec::new();
        while rest > 0 {
            digits.insert(0, (rest % 10) as u8);
            rest /= 10;
        }
        Password { digits }
    }
}

impl Password {
    fn valid_1(&self) -> bool {
        self.adjacent_matching_digits() && self.is_sorted()
    }

    fn valid_2(&self) -> bool {
        self.isolated_adjacent_matching_digits() && self.is_sorted()
    }

    fn adjacent_matching_digits(&self) -> bool {
        self.digits.windows(2).any(|w| { w[0] == w[1] })
    }

    fn isolated_adjacent_matching_digits(&self) -> bool {
        self.digits[0] == self.digits[1] && self.digits[1] != self.digits[2] ||
            self.digits[4] == self.digits[5] && self.digits[3] != self.digits[4] ||
            self.digits.windows(4).any(|w| { w[1] == w[2] && w[0] != w[1] && w[2] != w[3] })
    }

    fn is_sorted(&self) -> bool {
        self.digits.windows(2).all(|w| { w[0] <= w[1] })
    }
}

fn main() {
    let now = SystemTime::now();
    println!("Answer 1 = {}", count_passwords(Password::valid_1));
    println!("Answer 2 = {}", count_passwords(Password::valid_2));
    println!("{}ms", now.elapsed().unwrap().as_millis());
}

fn count_passwords(valid: fn(&Password) -> bool) -> usize {
    let range: Range<u32> = 264793..803935;
    range.map(Password::new).filter(valid).count()
}
