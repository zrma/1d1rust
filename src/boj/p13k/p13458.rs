use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13458(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let a: Vec<i64> = read_n_values(reader, n);
    let (b, c): (i64, i64) = read_values_as!(read_line(reader), i64, i64);

    let total: i64 = a.iter().fold(0, |acc, &x| {
        acc + 1 + if x > b { (x - b + c - 1) / c } else { 0 }
    });

    write!(writer, "{}", total).expect("write should work");
}

// https://www.acmicpc.net/problem/13458
// 시험 감독
#[test]
fn test_solve13458() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1
1
1 1"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3
3 4 5
2 2"
            .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "5
1000000 1000000 1000000 1000000 1000000
5 7"
            .to_string(),
            want: "714290".to_string(),
        },
        TestData {
            s: "5
10 9 10 9 10
7 20"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "5
10 9 10 9 10
7 2"
            .to_string(),
            want: "13".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve13458(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
