use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5361(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let prices = [350.34, 230.90, 190.55, 125.30, 180.90];

    let ans = (0..num_cases)
        .map(|_| {
            let parts = read_n_values::<f64>(reader, 5);
            let total = parts
                .iter()
                .zip(prices.iter())
                .map(|(p, price)| p * price)
                .sum::<f64>();
            format!("${:.2}", total)
        })
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/5361
// 전투 드로이드 가격
#[test]
fn test_solve5361() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
20 10 14 3 9
19 17 12 8 10
11 9 8 22 33"
                .to_string(),
            want: "$13987.50
$15679.76
$16182.54"
                .to_string(),
        },
        TestData {
            s: "1
1 1 1 1 1"
                .to_string(),
            want: "$1077.99".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5361(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
