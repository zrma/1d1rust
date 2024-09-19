use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1547(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let mut ball_position = 1;

    for _ in 0..num_cases {
        let (x, y) = read_values_as!(read_line(reader), usize, usize);
        if x == ball_position || y == ball_position {
            ball_position = if x == ball_position { y } else { x };
        }
    }

    write!(writer, "{}", ball_position).expect("Failed to write");
}

// https://www.acmicpc.net/problem/1547
// noinspection SpellCheckingInspection
// 공
#[test]
fn test_solve1547() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
3 1
2 3
3 1
3 2"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "2
1 2
3 1"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5
2 3
1 3
2 3
2 1
3 1"
            .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "9
1 2
3 2
1 2
2 1
2 1
3 2
1 3
3 1
1 2"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve1547(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
