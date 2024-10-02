use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2975(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];
    loop {
        let (balance, op, amount) = read_values_as!(read_line(reader), i32, char, i32);
        if balance == 0 && op == 'W' && amount == 0 {
            break;
        }

        let new_balance = match op {
            'W' => balance - amount,
            'D' => balance + amount,
            _ => panic!("Invalid operation"),
        };

        answers.push(if new_balance < -200 {
            "Not allowed".to_string()
        } else {
            new_balance.to_string()
        });
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/2975
// Transactions
#[test]
fn test_solve2975() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10000 W 10
-200 D 300
50 W 300
0 W 0"
                .to_string(),
            want: "9990
100
Not allowed"
                .to_string(),
        },
        TestData {
            s: "0 D -200
0 D -201
0 W 0"
                .to_string(),
            want: "-200
Not allowed"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2975(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
