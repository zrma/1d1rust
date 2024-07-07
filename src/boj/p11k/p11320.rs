use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11320(reader: &mut impl BufRead, writer: &mut impl Write) {
    count_cover_tris(reader, writer);
}

pub fn count_cover_tris(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let ans = (0..num_cases)
        .map(|_| {
            let (a, b) = read_values_as!(read_line(reader), i64, i64);
            let q = a / b;
            q * q
        })
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11320
// 삼각 무늬 - 1
#[test]
fn test_solve11320() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
2 1
3 3"
            .to_string(),
            want: "4
1"
            .to_string(),
        },
        TestData {
            s: "2
3 1
4 1"
            .to_string(),
            want: "9
16"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11320(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
