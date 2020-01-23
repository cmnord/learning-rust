use std::io::{self, Read};

#[allow(dead_code)]
fn jumping_on_clouds(n: usize, clouds: Vec<u8>) -> usize {
    println!("n = {}, clouds = {:?}", n, clouds);
    let mut min_steps = n;

    let mut queue: Vec<(usize, usize)> = vec![(0, 0)];
    while let Some((len, i)) = queue.pop() {
        println!("queue has {}, {}", len, i);
        // possible steps are +1 or +2
        if i + 1 < n && clouds[i + 1] == 0 {
            queue.push((len + 1, i + 1));
        }
        if i + 2 < n && clouds[i + 2] == 0 {
            queue.push((len + 1, i + 2))
        }
        if i == n - 1 {
            println!("end of the line...");
            if len < min_steps {
                println!("new record {}!", len);
                min_steps = len;
            }
        }
    }
    min_steps
}

fn _main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tokens: Vec<&str> = buffer.split("\n").collect();
    let n: usize = tokens[0].parse().unwrap();
    let clouds = tokens[1].split(" ").map(|c| c.parse().unwrap()).collect();

    let out = jumping_on_clouds(n, clouds);
    println!("{}", out);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::jumping_on_clouds;

    #[test]
    fn test_one() {
        assert_eq!(4, jumping_on_clouds(7, vec![0, 0, 1, 0, 0, 1, 0]));
    }
}
