fn look_and_say(s: &[u8]) -> Vec<u8> {
    let mut count = 0;
    let mut counted = 0;
    let mut r = Vec::new();

    for &c in s.iter().chain(std::iter::once(&0)) {
        if c == counted {
            count += 1;
        } else {
            if count > 0 {
                r.push(count);
                r.push(counted);
            }
            counted = c;
            count = 1;
        }
    }

    r
}

fn digits(s: &str) -> Vec<u8> {
    s.bytes().map(|c| c - b'0').collect()
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(adventofcode::read_input_file);

    let mut current = digits(&input);

    for _ in 0..40 {
        current = look_and_say(&current);
    }

    println!("{}", current.len());

    for _ in 0..10 {
        current = look_and_say(&current);
    }

    println!("{}", current.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test {
            t1("1", "11");
            t2("11", "21");
            t3("21", "1211");
            t4("1211", "111221");
            t5("111221", "312211");
        }
    }

    fn test(s: &str, expect: &str) {
        assert_eq!(look_and_say(&digits(s)), digits(expect));
    }
}
