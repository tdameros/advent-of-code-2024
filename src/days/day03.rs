use std::fs;
use regex::Regex;

pub fn solve() {
    let input = fs::read_to_string("assets/day03.txt").expect("Error reading the file");

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input).map(|cap| {
        let param1: i32 = cap[1].parse().unwrap();
        let param2: i32 = cap[2].parse().unwrap();
        param1 * param2
    }).sum()
}

fn solve_part_2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut is_do = true;

    re.captures_iter(input)
        .filter_map(|cap| {
            let is_mul = cap.get(1).is_some() && cap.get(2).is_some();
            if is_mul {
                let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                if is_do {
                    return Some(x * y);
                }
            } else if let Some(do_match) = cap.get(0) {
                if do_match.as_str() == "do()" {
                    is_do = true;
                } else if do_match.as_str() == "don't()" {
                    is_do = false;
                }
            }
            None

        })
        .sum()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve_part_1(input), 161);
    }

    #[test]
    fn test_example_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part_2(input), 48);
    }
}
