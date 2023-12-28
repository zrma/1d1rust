use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1924(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (month, day) = read_values!(read_line(reader), i32, i32);

    let days_per_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let total_days = days_per_month.iter().take(month as usize).sum::<i32>() + day;

    let weekdays = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let weekday = weekdays[total_days as usize % 7];

    write!(writer, "{}", weekday).unwrap();
}

// https://www.acmicpc.net/problem/1924
// 2007년
#[test]
fn test_solve1924() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1".to_string(),
            want: "MON".to_string(),
        },
        TestData {
            s: "3 14".to_string(),
            want: "WED".to_string(),
        },
        TestData {
            s: "9 2".to_string(),
            want: "SUN".to_string(),
        },
        TestData {
            s: "12 25".to_string(),
            want: "TUE".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1924(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
