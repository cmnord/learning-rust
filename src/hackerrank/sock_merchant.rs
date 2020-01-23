use std::io::{self, Read};

fn sock_merchant(num_socks: usize, mut socks: Vec<i32>) -> i32 {
    socks.sort();
    println!("socks is {:?}", socks);
    let mut matches = 0;

    let mut i: usize = 0;
    let mut j: usize = 1;

    while j < num_socks {
        println!("socks[{}] = {}, socks[{}] = {})", i, socks[i], j, socks[j]);
        if socks[i] == socks[j] {
            i += 2;
            j += 2;
            matches += 1;
        } else {
            i += 1;
            j += 1;
        }
    }
    matches
}

fn _main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let tokens: Vec<&str> = buffer.split("\n").collect();

    let num_socks: usize = tokens[0].parse().unwrap();
    let socks: Vec<i32> = tokens[1].split(" ").map(|s| s.parse().unwrap()).collect();

    let out = sock_merchant(num_socks, socks);
    println!("{}", out);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::sock_merchant;

    #[test]
    fn test_one() {
        assert_eq!(
            3,
            sock_merchant(9, vec![10, 20, 20, 10, 10, 30, 50, 10, 20])
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            6,
            sock_merchant(15, vec![6, 5, 2, 3, 5, 2, 2, 1, 1, 5, 1, 3, 3, 3, 5])
        );
    }
}
