use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28702(reader: &mut impl BufRead, writer: &mut impl Write) {
    let inputs = (0..3).map(|_| read_line(reader)).collect::<Vec<_>>();

    let mut current_number: Option<i32> = None;
    for input in &inputs {
        match input.as_str() {
            "Fizz" | "Buzz" | "FizzBuzz" => {
                if let Some(num) = current_number {
                    current_number = Some(num + 1);
                }
            }
            _ => {
                let number: i32 = input.parse().expect("Failed to parse");
                current_number = Some(number);
            }
        }
    }

    let next_num = current_number.expect("No valid number found") + 1;
    let ans = match (next_num % 3 == 0, next_num % 5 == 0) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        (false, false) => next_num.to_string(),
    };

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/28702
// noinspection SpellCheckingInspection
// FizzBuzz
#[test]
fn test_solve28702() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "Fizz
Buzz
11"
            .to_string(),
            want: "Fizz".to_string(),
        },
        TestData {
            s: "980803
980804
FizzBuzz"
                .to_string(),
            want: "980806".to_string(),
        },
        TestData {
            s: "1
2
Fizz"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2
Fizz
4"
            .to_string(),
            want: "Buzz".to_string(),
        },
        TestData {
            s: "Fizz
4
Buzz"
                .to_string(),
            want: "Fizz".to_string(),
        },
        TestData {
            s: "4
Buzz
Fizz"
                .to_string(),
            want: "7".to_string(),
        },
        TestData {
            s: "7
8
Fizz"
                .to_string(),
            want: "Buzz".to_string(),
        },
        TestData {
            s: "11
Fizz
13"
            .to_string(),
            want: "14".to_string(),
        },
        TestData {
            s: "FizzBuzz
16
17"
            .to_string(),
            want: "Fizz".to_string(),
        },
        TestData {
            s: "19
Buzz
Fizz"
                .to_string(),
            want: "22".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve28702(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
