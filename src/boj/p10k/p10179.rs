use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10179(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    let ans = (0..num_cases)
        .map(|_| {
            let price: f64 = read_value(read_line(reader));
            format!("${:.2}", price * 0.8)
        })
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/10179
// 쿠폰
#[test]
fn test_solve10179() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
100.00
59.99
20.00"
                .to_string(),
            want: "$80.00
$47.99
$16.00"
                .to_string(),
        },
        TestData {
            s: "1
0.10"
                .to_string(),
            want: "$0.08".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10179(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
