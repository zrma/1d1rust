use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25285(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let (height, weight) = read_values_as!(read_line(reader), u8, u8);
            assess_military_eligibility(height, bmi(weight, height)).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

fn bmi(weight: u8, height: u8) -> f32 {
    weight as f32 / (height as f32 / 100.0).powi(2)
}

fn assess_military_eligibility(height: u8, bmi: f32) -> u8 {
    match height {
        h if h >= 204 => 4,
        h if h >= 161 => match bmi {
            bmi if (20.0..25.0).contains(&bmi) => 1,
            bmi if (18.5..30.0).contains(&bmi) => 2,
            bmi if (16.0..35.0).contains(&bmi) => 3,
            _ => 4,
        },
        h if h >= 159 => match bmi {
            bmi if (16.0..35.0).contains(&bmi) => 3,
            _ => 4,
        },
        h if h >= 146 => 4,
        h if h > 140 => 5,
        _ => 6,
    }
}

// https://www.acmicpc.net/problem/25285
// 심준의 병역판정검사
#[test]
fn test_solve25285() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "6
175 65
175 60
175 49
175 48
145 50
140 45"
                .to_string(),
            want: "1
2
3
4
5
6"
            .to_string(),
        },
        TestData {
            s: "8
141 200
146 0
158 200
159 40
159 41
159 88
159 89
204 0
"
            .to_string(),
            want: "5
4
4
4
3
3
4
4"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25285(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
