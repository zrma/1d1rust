use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13223(reader: &mut impl BufRead, writer: &mut impl Write) {
    let start = parse_time_to_seconds(&read_line(reader));
    let end = parse_time_to_seconds(&read_line(reader));

    let diff = if end > start {
        end - start
    } else {
        24 * 3600 - start + end
    };

    writeln!(writer, "{}", format_seconds_to_time(diff)).expect("write! should work");
}

fn parse_time_to_seconds(time: &str) -> u32 {
    let mut parts = time
        .split(':')
        .map(|s| s.parse::<u32>().expect("time should be in HH:MM:SS format"));
    let h = parts.next().expect("missing hours");
    let m = parts.next().expect("missing minutes");
    let s = parts.next().expect("missing seconds");

    h * 3600 + m * 60 + s
}

fn format_seconds_to_time(seconds: u32) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    )
}

// https://www.acmicpc.net/problem/13223
// 소금 폭탄
#[test]
fn test_solve13223() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "20:00:00
04:00:00"
                .to_string(),
            want: "08:00:00".to_string(),
        },
        TestData {
            s: "12:34:56
14:36:22"
                .to_string(),
            want: "02:01:26".to_string(),
        },
        TestData {
            s: "00:00:00
00:00:00"
                .to_string(),
            want: "24:00:00".to_string(),
        },
        TestData {
            s: "23:59:59
23:59:59"
                .to_string(),
            want: "24:00:00".to_string(),
        },
        TestData {
            s: "23:59:59
00:00:00"
                .to_string(),
            want: "00:00:01".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve13223(&mut reader, &mut writer);

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
