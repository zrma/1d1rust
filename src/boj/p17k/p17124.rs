use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17124(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut answers = Vec::new();
    for _ in 0..num_cases {
        let (num_a, num_b) = read_values_as!(read_line(reader), usize, usize);

        let a_values: Vec<i64> = read_n_values(reader, num_a);
        let mut b_values: Vec<i64> = read_n_values(reader, num_b);
        b_values.sort_unstable();

        let total_closest_sum: i64 = a_values.iter().map(|&ai| find_closest(&b_values, ai)).sum();
        answers.push(total_closest_sum.to_string());
    }

    writeln!(writer, "{}", answers.join("\n")).unwrap();
}

fn find_closest(b_values: &[i64], target: i64) -> i64 {
    let mut left = 0;
    let mut right = b_values.len();
    while left < right {
        let mid = (left + right) / 2;
        if b_values[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left == 0 {
        b_values[left]
    } else if left == b_values.len() || target - b_values[left - 1] <= b_values[left] - target {
        b_values[left - 1]
    } else {
        b_values[left]
    }
}

// https://www.acmicpc.net/problem/17124
// 두 개의 배열
#[test]
fn test_solve17124() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
4 3
20 5 14 9
16 8 12
3 4
16 8 12
20 5 14 9
3 3
1 2 3
2 3 4"
                .to_string(),
            want: "44
37
7"
            .to_string(),
        },
        TestData {
            s: "2
5 2
1 2 3 4 50
1 100
5 2
1 2 3 4 51
1 100"
                .to_string(),
            want: "5
104"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve17124(&mut reader, &mut writer);

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
