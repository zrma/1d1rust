use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2506(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_questions = read_value(read_line(reader));
    let scores: Vec<i32> = read_value::<String>(read_line(reader))
        .split_whitespace()
        .take(num_questions)
        .map(|s| s.parse().unwrap())
        .collect::<_>();

    let (total_score, _) = scores.iter().fold((0, 0), |(acc_score, streak), &result| {
        if result == 1 {
            let new_streak = streak + 1;
            (acc_score + new_streak, new_streak)
        } else {
            (acc_score, 0)
        }
    });

    write!(writer, "{}", total_score).expect("write! should work");
}

// https://www.acmicpc.net/problem/2506
// noinspection SpellCheckingInspection
// 점수계산
#[test]
fn test_solve2506() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10
1 0 1 1 1 0 0 1 1 0"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "3
1 1 1"
                .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "7
1 0 1 1 1 0 1"
                .to_string(),
            want: "8".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2506(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
