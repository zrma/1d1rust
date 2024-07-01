use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4435(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(t);
    for i in 1..=t {
        let good = read_n_values::<i32>(reader, 6);
        let evil = read_n_values::<i32>(reader, 7);

        let good_power_weights = [1, 2, 3, 3, 4, 10];
        let evil_power_weights = [1, 2, 2, 2, 3, 5, 10];

        let good_power_sum: i32 = good
            .iter()
            .zip(good_power_weights.iter())
            .map(|(a, b)| a * b)
            .sum();
        let evil_power_sum: i32 = evil
            .iter()
            .zip(evil_power_weights.iter())
            .map(|(a, b)| a * b)
            .sum();

        let ans = match good_power_sum.cmp(&evil_power_sum) {
            std::cmp::Ordering::Greater => "Good triumphs over Evil",
            std::cmp::Ordering::Less => "Evil eradicates all trace of Good",
            std::cmp::Ordering::Equal => "No victor on this battle field",
        };

        answers.push(format!("Battle {}: {}", i, ans));
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4435
// 중간계 전쟁
#[test]
fn test_solve4435() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1 1 1 1 1 1
1 1 1 1 1 1 1
0 0 0 0 0 10
0 1 1 1 1 0 0
1 0 0 0 0 0
1 0 0 0 0 0 0"
                .to_string(),
            want: "Battle 1: Evil eradicates all trace of Good
Battle 2: Good triumphs over Evil
Battle 3: No victor on this battle field"
                .to_string(),
        },
        TestData {
            s: "1
1 1 1 1 1 1
1 1 1 0 1 1 1"
                .to_string(),
            want: "Battle 1: No victor on this battle field".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4435(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
