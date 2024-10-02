use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10707(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a: i32 = read_value(read_line(reader));
    let b: i32 = read_value(read_line(reader));
    let c: i32 = read_value(read_line(reader));
    let d: i32 = read_value(read_line(reader));
    let p: i32 = read_value(read_line(reader));

    let x_cost = a * p;
    let y_cost = if p <= c { b } else { b + (p - c) * d };
    let ans = x_cost.min(y_cost);

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/10707
// 수도요금
#[test]
fn test_solve10707() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9
100
20
3
10"
            .to_string(),
            want: "90".to_string(),
        },
        TestData {
            s: "8
300
100
10
250"
            .to_string(),
            want: "1800".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10707(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
