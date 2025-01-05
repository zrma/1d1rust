use crate::read_values_as;
use crate::utils::io::{matrix_to_str, read_line};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15666(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);
    let mut unique_nums = extract_unique_numbers(reader, n);
    unique_nums.sort_unstable();

    let combinations = generate_combinations(&unique_nums, m);
    let ans = matrix_to_str(&combinations);

    writeln!(writer, "{}", ans).unwrap();
}

fn extract_unique_numbers(reader: &mut impl BufRead, n: usize) -> Vec<usize> {
    let mut set = std::collections::HashSet::new();
    read_line(reader)
        .split_whitespace()
        .take(n)
        .filter_map(|s| s.parse::<usize>().ok())
        .filter(|&num| set.insert(num))
        .collect()
}

fn generate_combinations(nums: &Vec<usize>, m: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    generate_combinations_helper(nums, &mut vec![], &mut result, m, 0);
    result
}

fn generate_combinations_helper(
    nums: &Vec<usize>,
    picked: &mut Vec<usize>,
    result: &mut Vec<Vec<usize>>,
    m: usize,
    idx: usize,
) {
    if picked.len() == m {
        result.push(picked.clone());
        return;
    }

    for i in idx..nums.len() {
        picked.push(nums[i]);
        generate_combinations_helper(nums, picked, result, m, i);
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
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
