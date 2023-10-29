use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16479(reader: &mut impl BufRead, writer: &mut impl Write) {
    let k = read_line(reader).parse::<i32>().unwrap();
    let (d1, d2) = read_values!(read_line(reader), i32, i32);

    let res = match d1.cmp(&d2) {
        std::cmp::Ordering::Equal => k.pow(2) as f64,
        std::cmp::Ordering::Less => {
            let d3 = d2 - d1;
            k.pow(2) as f64 - (d3.pow(2) as f64) / 4.0
        }
        std::cmp::Ordering::Greater => {
            let d3 = d1 - d2;
            k.pow(2) as f64 - (d3.pow(2) as f64) / 4.0
        }
    };

    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/16479
// 컵라면 측정하기
#[test]
fn test_solve16479() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "14
12 12"
                .to_string(),
            want: "196".to_string(),
        },
        TestData {
            s: "8
9 3"
            .to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "15
13 6"
                .to_string(),
            want: "212.75".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16479(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
