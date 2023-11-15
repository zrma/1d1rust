use crate::read_values;
use crate::utils::io::{matrix_to_str, read_line};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15666(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values!(read_line(reader), usize, usize);
    let mut unique_nums = extract_unique_numbers(reader, n);
    unique_nums.sort_unstable();

    let result = generate_combinations(&unique_nums, m);
    let result_str = matrix_to_str(&result);

    write!(writer, "{}", result_str).expect("Failed to write");
}

fn extract_unique_numbers(reader: &mut impl BufRead, n: usize) -> Vec<i32> {
    let mut set = std::collections::HashSet::new();
    read_line(reader)
        .split_whitespace()
        .take(n)
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&num| set.insert(num))
        .collect()
}

fn generate_combinations(nums: &Vec<i32>, m: usize) -> Vec<Vec<i32>> {
    let mut result = vec![];
    generate_combinations_helper(nums, &mut vec![], &mut result, m, 0);
    result
}

fn generate_combinations_helper(
    nums: &Vec<i32>,
    picked: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
    m: usize,
    idx: i32,
) {
    if picked.len() == m {
        result.push(picked.clone());
        return;
    }

    for i in idx as usize..nums.len() {
        picked.push(nums[i]);
        generate_combinations_helper(nums, picked, result, m, i as i32);
        picked.pop();
    }
}

// https://www.acmicpc.net/problem/15666
// N과 M (12)
#[test]
fn test_solve15666() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 1
4 4 2"
                .to_string(),
            want: "2
4"
            .to_string(),
        },
        TestData {
            s: "4 2
9 7 9 1"
                .to_string(),
            want: "1 1
1 7
1 9
7 7
7 9
9 9"
            .to_string(),
        },
        TestData {
            s: "4 4
1 1 2 2"
                .to_string(),
            want: "1 1 1 1
1 1 1 2
1 1 2 2
1 2 2 2
2 2 2 2"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15666(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
