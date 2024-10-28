use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering::{Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5523(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let (a_wins, b_wins) = (0..n).fold((0, 0), |(a_wins, b_wins), _| {
        let (a, b) = read_values_as!(read_line(reader), i32, i32);
        match a.cmp(&b) {
            Greater => (a_wins + 1, b_wins),
            Less => (a_wins, b_wins + 1),
            _ => (a_wins, b_wins),
        }
    });

    write!(writer, "{} {}", a_wins, b_wins).expect("write! should work");
}

// https://www.acmicpc.net/problem/5523
// 경기 결과
#[test]
fn test_solve5523() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
100 0
5 6
40 50
74 75"
                .to_string(),
            want: "1 3".to_string(),
        },
        TestData {
            s: "5
20 20
3 95
60 59
40 40
20 19"
                .to_string(),
            want: "2 1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5523(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
