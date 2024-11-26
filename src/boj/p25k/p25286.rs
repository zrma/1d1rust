use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25286(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let (y, m) = read_values_as!(read_line(reader), i32, usize);
            let (new_y, prev_m, last_day) = calculate_date(y, m);
            format!("{} {} {}", new_y, prev_m, last_day)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn calculate_date(year: i32, month: usize) -> (i32, usize, usize) {
    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if is_leap_year(year) {
        days_in_month[1] = 29;
    }

    if month == 1 {
        (year - 1, 12, 31)
    } else {
        (year, month - 1, days_in_month[month - 2])
    }
}

// https://www.acmicpc.net/problem/25286
// 11월 11일
#[test]
fn test_solve25286() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "6
2000 3
2012 3
2022 3
2100 3
2022 1
2022 11"
                .to_string(),
            want: "2000 2 29
2012 2 29
2022 2 28
2100 2 28
2021 12 31
2022 10 31"
                .to_string(),
        },
        TestData {
            s: "1
2024 11"
                .to_string(),
            want: "2024 10 31".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25286(&mut reader, &mut writer);

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
