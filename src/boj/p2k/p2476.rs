use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::collections::HashMap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2476(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value::<usize>(read_line(reader));
    let max_prize = (0..num_cases)
        .map(|_| {
            let (die1, die2, die3) = read_values_as!(read_line(reader), u32, u32, u32);
            calculate_prize(die1, die2, die3)
        })
        .max()
        .unwrap();

    write!(writer, "{}", max_prize).expect("Failed to write");
}

fn calculate_prize(die1: u32, die2: u32, die3: u32) -> u32 {
    let mut counts = HashMap::new();
    *counts.entry(die1).or_insert(0) += 1;
    *counts.entry(die2).or_insert(0) += 1;
    *counts.entry(die3).or_insert(0) += 1;

    match counts.iter().max_by_key(|&(_, count)| count) {
        Some((&num, &3)) => 10000 + num * 1000,
        Some((&num, &2)) => 1000 + num * 100,
        Some((_, &1)) => *counts.keys().max().unwrap() * 100,
        _ => unreachable!(),
    }
}

// https://www.acmicpc.net/problem/2476
// noinspection SpellCheckingInspection
// 주사위 게임
#[test]
fn test_solve2476() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
3 3 6
2 2 2
6 2 5"
                .to_string(),
            want: "12000".to_string(),
        },
        TestData {
            s: "1
6 6 6"
                .to_string(),
            want: "16000".to_string(),
        },
        TestData {
            s: "1
1 1 1"
                .to_string(),
            want: "11000".to_string(),
        },
        TestData {
            s: "1
1 1 2"
                .to_string(),
            want: "1100".to_string(),
        },
        TestData {
            s: "1
1 2 2"
                .to_string(),
            want: "1200".to_string(),
        },
        TestData {
            s: "1
1 2 3"
                .to_string(),
            want: "300".to_string(),
        },
        TestData {
            s: "3
1 2 3
4 5 6
1 1 1"
                .to_string(),
            want: "11000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2476(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
