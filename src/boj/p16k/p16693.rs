use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16693(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a1, p1) = read_values_as!(read_line(reader), f64, f64);
    let (r1, p2) = read_values_as!(read_line(reader), f64, f64);

    let ratio1 = a1 / p1;
    let ratio2 = std::f64::consts::PI * r1 * r1 / p2;

    if ratio1 > ratio2 {
        write!(writer, "Slice of pizza").expect("Failed to write");
    } else {
        write!(writer, "Whole pizza").expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/16693
// Pizza 2
#[test]
fn test_solve16693() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8 4
7 9"
            .to_string(),
            want: "Whole pizza".to_string(),
        },
        TestData {
            s: "9 2
4 7"
            .to_string(),
            want: "Whole pizza".to_string(),
        },
        TestData {
            s: "841 108
8 606"
                .to_string(),
            want: "Slice of pizza".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve16693(reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
