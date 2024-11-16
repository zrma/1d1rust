use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29752(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let values: Vec<usize> = read_n_values(reader, n);

    let (max_streak, _) = values
        .iter()
        .fold((0, 0), |(max_streak, curr_streak), &val| {
            if val > 0 {
                (max_streak.max(curr_streak + 1), curr_streak + 1)
            } else {
                (max_streak, 0)
            }
        });

    writeln!(writer, "{}", max_streak).expect("write should work");
}

// https://www.acmicpc.net/problem/29752
// 최장 스트릭
#[test]
fn test_solve29752() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
1 4 0 1"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5
1 6 3 8 3"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "3
0 100 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1
0"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve29752(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got.trim(), data.want, "failed at {} with {}", i, data.s);
    }
}
