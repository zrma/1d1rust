use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20206(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), f32, f32, f32);
    let (x_min, x_max, y_min, y_max) = read_values_as!(read_line(reader), f32, f32, f32, f32);

    let check_inside_range = |val, min, max| min < val && val < max;
    let ans = if a == 0.0 {
        check_inside_range(-(c / b), y_min, y_max)
    } else if b == 0.0 {
        check_inside_range(-(c / a), x_min, x_max)
    } else {
        let y1 = -(a * x_min + c) / b;
        let y2 = -(a * x_max + c) / b;

        if a / b < 0.0 {
            y_min < y2 && y1 < y_max
        } else {
            y_min < y1 && y2 < y_max
        }
    };

    write!(writer, "{}", if ans { "Poor" } else { "Lucky" }).unwrap();
}

// https://www.acmicpc.net/problem/20206
// 푸앙이가 길을 건너간 이유
#[test]
fn test_solve20206() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 4 5
-1 0 -1 0"
                .to_string(),
            want: "Poor".to_string(),
        },
        TestData {
            s: "1 1 -2
-1 1 -1 1"
                .to_string(),
            want: "Lucky".to_string(),
        },
        TestData {
            s: "0 1 -1
0 1 0 1"
                .to_string(),
            want: "Lucky".to_string(),
        },
        TestData {
            s: "0 1 -1
0 1 0 2"
                .to_string(),
            want: "Poor".to_string(),
        },
        TestData {
            s: "1 0 -1
0 1 0 1"
                .to_string(),
            want: "Lucky".to_string(),
        },
        TestData {
            s: "1 0 -1
0 2 0 1"
                .to_string(),
            want: "Poor".to_string(),
        },
        TestData {
            s: "-1 1 0
-1 1 -1 1"
                .to_string(),
            want: "Poor".to_string(),
        },
        TestData {
            s: "1 -1 0
-1 1 -1 1"
                .to_string(),
            want: "Poor".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve20206(&mut reader, &mut writer);

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
