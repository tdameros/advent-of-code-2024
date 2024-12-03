use std::fs;

pub fn solve() {
    let input = fs::read_to_string("assets/day02.txt").expect("Error reading the file");
    let rows = input.split("\n");
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for row in rows {
        let numbers: Vec<i32> = row
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        reports.push(numbers);
    }
    println!("Part 1: {}", solve_part_1(&reports));
    println!("Part 2: {}", solve_part_2(&reports));
}

fn solve_part_1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for report in reports {
        if report.len() < 2 {
            continue;
        }
        if is_safe_report(report) {
            result += 1;
        }
    }
    result
}

fn solve_part_2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for report in reports {
        if report.len() < 2 {
            continue;
        }
        for i in 0..report.len() {
            if is_safe_report_with_remove_index(report, i as u32) {
                result += 1;
                break;
            }
        }
    }
    result
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let is_increase = match report[0].cmp(&report[1]) {
        std::cmp::Ordering::Less => true,
        std::cmp::Ordering::Greater => false,
        std::cmp::Ordering::Equal => return false,
    };
    for i in 1..report.len() {
        if (is_increase && (report[i] - report[i - 1] < 1 || report[i] - report[i - 1] > 3)) ||
            (!is_increase && (report[i - 1] - report[i] < 1 || report[i - 1] - report[i] > 3)) {
            return false;
        }
    }
    true
}

fn is_safe_report_with_remove_index(report: &Vec<i32>, remove_index: u32) -> bool {
    let (sub, mut last_index, start_index) = match remove_index {
        0 => (report[1] - report[2], 1, 2),
        1 => (report[0] - report[2], 0, 2),
        _ => (report[0] - report[1], 0, 1),
    };

    let is_increase = match sub {
        x if x > 0 => false,
        x if x < 0 => true,
        _ => return false,
    };

    for i in start_index..report.len() {
        if i == remove_index as usize {
            continue;
        }
        if (is_increase && (report[i] - report[last_index] < 1 || report[i] - report[last_index] > 3)) ||
            (!is_increase && (report[last_index] - report[i] < 1 || report[last_index] - report[i] > 3)) {
            return false;
        }
        last_index = i;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_part_1() {
        // 7 6 4 2 1
        // 1 2 7 8 9
        // 9 7 6 2 1
        // 1 3 2 4 5
        // 8 6 4 4 1
        // 1 3 6 7 9
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(solve_part_1(&reports), 2);
    }

    #[test]
    fn test_example_part_2() {
        // 7 6 4 2 1
        // 1 2 7 8 9
        // 9 7 6 2 1
        // 1 3 2 4 5
        // 8 6 4 4 1
        // 1 3 6 7 9
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(solve_part_2(&reports), 4);
    }
}
