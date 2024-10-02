use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2455(reader: &mut impl BufRead, writer: &mut impl Write) {
    let max_passengers = calc_max_passengers(reader, 4);

    write!(writer, "{}", max_passengers).expect("Failed to write");
}

pub fn calc_max_passengers(reader: &mut impl BufRead, n: usize) -> i32 {
    let (max_passengers, _) = (0..n).fold((0, 0), |(max, curr), _| {
        let (people_out, people_in) = read_values_as!(read_line(reader), i32, i32);
        let new_current = curr + people_in - people_out;
        (max.max(new_current), new_current)
    });

    max_passengers
}

// https://www.acmicpc.net/problem/2455
// noinspection SpellCheckingInspection
// 지능형 기차
#[test]
fn test_solve2455() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "0 32
3 13
28 25
39 0"
                .to_string(),
            want: "42".to_string(),
        },
        TestData {
            s: "0 1
0 0
0 0
1 0"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "0 100
100 0
0 99
99 0"
                .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "0 99
99 0
0 100
100 0"
                .to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2455(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
