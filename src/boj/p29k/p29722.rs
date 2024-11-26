use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29722(reader: &mut impl BufRead, writer: &mut impl Write) {
    let date_str = read_line(reader);
    let frequency: i32 = read_value(read_line(reader));

    let (year, month, day) = parse_date(&date_str);
    let total_days_current = to_total_days(year, month, day);
    let total_days_target = total_days_current + frequency;

    let (new_year, new_month, new_day) = from_total_days(total_days_target);

    writeln!(writer, "{:04}-{:02}-{:02}", new_year, new_month, new_day).expect("write should work");
}

fn parse_date(date_str: &str) -> (i32, i32, i32) {
    let mut date_parts = date_str
        .split('-')
        .map(|s| s.parse::<i32>().expect("Invalid date format"));
    (
        date_parts.next().expect("Missing year"),
        date_parts.next().expect("Missing month"),
        date_parts.next().expect("Missing day"),
    )
}

fn to_total_days(year: i32, month: i32, day: i32) -> i32 {
    year * 360 + (month - 1) * 30 + (day - 1)
}

fn from_total_days(total_days: i32) -> (i32, i32, i32) {
    let year = total_days / 360;
    let remaining_days = total_days % 360;
    let month = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    (year, month, day)
}

// https://www.acmicpc.net/problem/29722
// 브실혜성
#[test]
fn test_solve29722() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2023-07-08
30"
            .to_string(),
            want: "2023-08-08".to_string(),
        },
        TestData {
            s: "2023-07-08
720"
            .to_string(),
            want: "2025-07-08".to_string(),
        },
        TestData {
            s: "2022-11-30
30"
            .to_string(),
            want: "2022-12-30".to_string(),
        },
        TestData {
            s: "1000-12-30
331"
            .to_string(),
            want: "1001-12-01".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve29722(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
