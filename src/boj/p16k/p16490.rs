use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16490(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, t) = read_values_as!(read_line(reader), i32, i32);
    let ans = a * a - t * t;
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/16490
// 외계인의 침투
#[test]
fn test_solve16490() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9 8".to_string(),
            want: "17".to_string(),
        },
        TestData {
            s: "100 9".to_string(),
            want: "9919".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16490(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
