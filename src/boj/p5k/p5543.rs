use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5543(reader: &mut impl BufRead, writer: &mut impl Write) {
    let burgers = (0..3).map(|_| read_price(reader)).min().unwrap();
    let drinks = (0..2).map(|_| read_price(reader)).min().unwrap();

    let total_cost = burgers + drinks - 50;
    write!(writer, "{}", total_cost).unwrap();
}

fn read_price(reader: &mut impl BufRead) -> i32 {
    read_value::<i32>(read_line(reader))
}

// https://www.acmicpc.net/problem/5543
// 상근날드
#[test]
fn test_solve5543() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "800
700
900
198
330"
            .to_string(),
            want: "848".to_string(),
        },
        TestData {
            s: "1999
1999
100
189
100"
            .to_string(),
            want: "150".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5543(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
