use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29751(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (w, h) = read_values_as!(read_line(reader), f64, f64);

    let ans = (w * h) / 2.0;
    write!(writer, "{:.1}", ans).unwrap();
}

// https://www.acmicpc.net/problem/29751
// 삼각형
#[test]
fn test_solve29751() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1".to_string(),
            want: "0.5".to_string(),
        },
        TestData {
            s: "2 3".to_string(),
            want: "3.0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve29751(&mut reader, &mut writer);

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
