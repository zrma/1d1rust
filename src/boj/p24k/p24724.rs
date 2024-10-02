use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24724(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|i| {
            let num_lines: usize = read_value(read_line(reader));

            for _ in 0..=num_lines {
                let (_, _): (usize, usize) = read_values_as!(read_line(reader), usize, usize);
            }

            format!("Material Management {}\nClassification ---- End!", i + 1)
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/24724
// 현대모비스와 함께하는 부품 관리
#[test]
fn test_solve24724() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "2
3
4 5
2 3
1 4
3 1
2
2 2
2 2
2 2"
            .to_string(),
            want: "Material Management 1
Classification ---- End!
Material Management 2
Classification ---- End!"
                .to_string(),
        },
        TestData {
            s: "1
1
1 1
2 1"
            .to_string(),
            want: "Material Management 1
Classification ---- End!"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve24724(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
