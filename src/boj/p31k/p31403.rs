use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve31403(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a: i32 = read_value(read_line(reader));
    let b: i32 = read_value(read_line(reader));
    let c: i32 = read_value(read_line(reader));

    let simple_sum = a + b - c;
    let concatenated_sum = format!("{}{}", a, b).parse::<i32>().unwrap() - c;

    writeln!(writer, "{}", simple_sum).expect("Failed to write");
    write!(writer, "{}", concatenated_sum).expect("Failed to write");
}

// https://www.acmicpc.net/problem/31403
// A + B - C
#[test]
fn test_solve31403() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
4
5"
            .to_string(),
            want: "2
29"
            .to_string(),
        },
        TestData {
            s: "5
4
3"
            .to_string(),
            want: "6
51"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve31403(&mut reader, &mut writer);

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
