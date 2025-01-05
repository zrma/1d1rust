use crate::utils::io::read_values;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17284(reader: &mut impl BufRead, writer: &mut impl Write) {
    let button_presses = read_values(reader);

    let cost: i32 = button_presses
        .iter()
        .map(|&x| match x {
            1 => 500,
            2 => 800,
            3 => 1000,
            _ => unreachable!(),
        })
        .sum();

    let ans = 5000 - cost;
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/17284
// Vending Machine
#[test]
fn test_solve17284() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2 3".to_string(),
            want: "2700".to_string(),
        },
        TestData {
            s: "1 1 1".to_string(),
            want: "3500".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve17284(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
