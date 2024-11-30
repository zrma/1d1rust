use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5063(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    for _ in 0..n {
        let (r, e, c) = read_values_as!(read_line(reader), i32, i32, i32);
        let net_profit = e - c;

        let ans = match net_profit.cmp(&r) {
            Ordering::Greater => "advertise",
            Ordering::Equal => "does not matter",
            Ordering::Less => "do not advertise",
        };

        writeln!(writer, "{}", ans).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/5063
// TGN
#[test]
fn test_solve5063() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
0 100 70
100 130 30
-100 -70 40"
                .to_string(),
            want: "advertise
does not matter
do not advertise
"
            .to_string(),
        },
        TestData {
            s: "1
-100 0 100"
                .to_string(),
            want: "does not matter
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5063(&mut reader, &mut writer);

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
