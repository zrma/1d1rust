use crate::read_values_as;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5666(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        let (h, p) = read_values_as!(&line, f64, f64);
        let ans = h / p;
        writeln!(writer, "{:.2}", ans).expect("writeln! should work");
        line.clear();
    }
}

// https://www.acmicpc.net/problem/5666
// Hot Dogs
#[test]
fn test_solve5666() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10 90
840 11
1 50
33 1000
34 1000
36 1000
37 1000
1 1000"
                .to_string(),
            want: "0.11
76.36
0.02
0.03
0.03
0.04
0.04
0.00"
                .to_string(),
        },
        TestData {
            s: "100 100".to_string(),
            want: "1.00".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5666(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        let want = data.want.split_whitespace();

        use crate::utils::assert::match_multilines_as_f64;
        match_multilines_as_f64(i, got.split_whitespace(), want);
    }
}
