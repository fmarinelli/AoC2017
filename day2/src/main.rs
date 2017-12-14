use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = "5 1 9 5\n\
                        7 5 3\n\
                        2 4 6 8\n".to_string();
    let sum = calculate_checksum(&input);
    assert_eq!(sum, 18);

    let mut f = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    println!("Checksum {}", calculate_checksum(&contents));


    let input = "5 9 2 8\n\
    9 4 7 3\n\
    3 8 6 5".to_string();
    let sum = calculate_evenly_divisible(&input);
    assert_eq!(sum, 9);

    println!("Evenly divisible {}", calculate_evenly_divisible(&contents));
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_half() {
        let input = "5 1 9 5\n\
                        7 5 3\n\
                        2 4 6 8\n".to_string();
        let sum = calculate_checksum(&input);
        assert_eq!(sum, 18);
    }

    fn second_half() {}
}

fn find_divisible(v: &Vec<u32>) -> Result<(u32, u32), &'static str> {
    if v.len()<=1 {
        Err("Divisible not found")
    }
    else {
        match v[1..].iter().find(|&x| ((v[0] % x) == 0))  {
            Some(&y) => Ok( (v[0], y) ),
            _ => find_divisible(&v[1..].to_vec())
        }
    }
}

pub fn calculate_evenly_divisible(contents: &String) -> u32 {
    let mut m: Vec<Vec<u32>> = contents.lines()
        .map(|line| line.split_whitespace().map(|c| c.parse().unwrap()).collect())
        .collect();

    m.iter_mut()
        .map(|v| {
            v.sort_by(|a, b| b.cmp(a));
            v
        })
        .fold(0, |sum, v| {
            match find_divisible(&v)  {
                Ok((a, b)) => sum + a/b,
                Err(_) => sum
            }
        })
}

pub fn calculate_checksum(contents: &String) -> u32 {
    contents.lines().enumerate()
        .fold(0, |sum, (_, line)| {
            let values: Vec<u32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();
            let max = values.iter().max().unwrap();
            let min = values.iter().min().unwrap();
            sum + (max - min)
        })
}