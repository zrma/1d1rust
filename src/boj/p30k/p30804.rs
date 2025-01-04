use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30804(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let fruits: Vec<usize> = read_n_values(reader, n);

    writeln!(writer, "{}", max_continuous_fruits(&fruits)).unwrap();
}

fn max_continuous_fruits(fruits: &[usize]) -> usize {
    let mut max_len = 0;
    let mut count = [0; 10]; // 과일 종류는 1~9
    let mut unique = 0;
    let mut left = 0;

    for (right, &fruit) in fruits.iter().enumerate() {
        if count[fruit] == 0 {
            unique += 1;
        }
        count[fruit] += 1;

        while unique > 2 {
            let left_fruit = fruits[left];
            count[left_fruit] -= 1;
            if count[left_fruit] == 0 {
                unique -= 1;
            }
            left += 1;
        }

        max_len = max_len.max(right - left + 1);
    }

    max_len
}

// https://www.acmicpc.net/problem/30804
// 과일 탕후루
#[test]
fn test_solve30804() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
5 1 1 2 1"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3
1 1 1"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "9
1 2 3 4 5 6 7 8 9"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve30804(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("valid utf8 string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
