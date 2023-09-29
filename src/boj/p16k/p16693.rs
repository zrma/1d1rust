use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16693(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a1, p1) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    };

    let (r1, p2) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        (
            iter.next().unwrap().parse::<f64>().unwrap(),
            iter.next().unwrap().parse::<f64>().unwrap(),
        )
    };

    let ratio1 = a1 / p1;
    let ratio2 = std::f64::consts::PI * r1 * r1 / p2;

    if ratio1 > ratio2 {
        write!(writer, "Slice of pizza").unwrap();
    } else {
        write!(writer, "Whole pizza").unwrap();
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
    for (i, data) in vec![
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
