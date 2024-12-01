use std::fs;

pub fn solve() {
    let input = fs::read_to_string("assets/day01.txt").expect("Error reading the file");
    let rows = input.split("\n");
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for row in rows {
        let numbers: Vec<i32> = row
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }
    left_list.sort();
    right_list.sort();
    println!("Part 1: {}", solve_part_1(&left_list, &right_list));
    println!("Part 2: {}", solve_part_2(&left_list, &right_list));
}

fn solve_part_1(left_list: &[i32], right_list: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..left_list.len() {
        result += (right_list[i] - left_list[i]).abs();
    }
    result
}

fn solve_part_2(left_list: &[i32], right_list: &[i32]) -> i32 {
    let mut result = 0;
    for number in left_list {
        let count = number * count_element_in_sorted_list(right_list, *number);
        result += count;
    }
    result
}

fn count_element_in_sorted_list(list: &[i32], element: i32) -> i32 {
    if let Ok(index) = list.binary_search(&element) {
        let mut start = index;
        let mut end = index;

        while start > 0 && list[start - 1] == element {
            start -= 1;
        }
        while end < list.len() && list[end + 1] == element {
            end += 1;
        }
        (end - start + 1) as i32
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_part_1() {
        // 3   4
        // 4   3
        // 2   5
        // 1   3
        // 3   9
        // 3   3
        let mut left_list = vec![3, 4, 2, 1, 3, 3];
        let mut right_list = vec![4, 3, 5, 3, 9, 3];
        left_list.sort();
        right_list.sort();
        assert_eq!(solve_part_1(&left_list, &right_list), 11);
    }

    #[test]
    fn test_example_part_2() {
        // 3   4
        // 4   3
        // 2   5
        // 1   3
        // 3   9
        // 3   3
        let mut left_list = vec![3, 4, 2, 1, 3, 3];
        let mut right_list = vec![4, 3, 5, 3, 9, 3];
        left_list.sort();
        right_list.sort();
        assert_eq!(solve_part_2(&left_list, &right_list), 31);
    }
}
