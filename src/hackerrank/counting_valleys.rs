use std::io::{self, Read};

#[allow(dead_code)]
fn counting_valleys(steps: &str) -> i32 {
    let mut height = 0;
    let mut in_valley = false;
    let mut num_valleys = 0;

    for step in steps.chars() {
        height += match step {
            'U' => 1,
            'D' => -1,
            _ => 0,
        };
        println!("height is now {}, in_valley = {}", height, in_valley);
        if in_valley && height == 0 {
            num_valleys += 1;
            in_valley = false;
        } else if !in_valley && height < 0 {
            in_valley = true;
        }
    }
    num_valleys
}

fn _main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let tokens: Vec<&str> = buffer.split("\n").collect();

    let out = counting_valleys(tokens[1]);
    println!("{}", out);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::counting_valleys;

    #[test]
    fn test_one() {
        assert_eq!(1, counting_valleys("UDDDUDUU"));
    }

    #[test]
    fn test_two() {
        assert_eq!(1, counting_valleys("DDUUUUDD"));
    }

    #[test]
    fn test_three() {
        assert_eq!(1, counting_valleys("DU"));
        assert_eq!(1, counting_valleys("DDUU"));
        assert_eq!(1, counting_valleys("DDDUUUD"));
    }

    #[test]
    fn test_four() {
        assert_eq!(0, counting_valleys("UDUUUDUDDD"));
    }
}
