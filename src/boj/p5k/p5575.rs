use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5575(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut work_durations = vec![];

    for _ in 0..3 {
        let (start_hr, start_min, start_sec, end_hr, end_min, end_sec) =
            read_values_as!(read_line(reader), i32, i32, i32, i32, i32, i32);
        let total_seconds =
            (end_hr - start_hr) * 3600 + (end_min - start_min) * 60 + (end_sec - start_sec);
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        work_durations.push(format!("{} {} {}", hours, minutes, seconds));
    }

    writeln!(writer, "{}", work_durations.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/5575
// noinspection SpellCheckingInspection
// 타임 카드
#[test]
fn test_solve5575() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9 0 0 18 0 0
9 0 1 18 0 0
12 14 52 12 15 30"
                .to_string(),
            want: "9 0 0
8 59 59
0 0 38"
                .to_string(),
        },
        TestData {
            s: "7 0 0 16 0 0
7 0 1 16 0 0
7 0 0 16 0 1"
                .to_string(),
            want: "9 0 0
8 59 59
9 0 1"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve5575(&mut reader, &mut writer);

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
