use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11121(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_cases = read_value(read_line(reader));
    let mut answers = Vec::with_capacity(num_of_cases);

    for _ in 0..num_of_cases {
        let (a, b) = read_values_as!(read_line(reader), String, String);
        let ans = if a == b { "OK" } else { "ERROR" };
        answers.push(ans);
    }

    let output = answers.join("\n");
    write!(writer, "{}", output).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11121
// Communication Channels
#[test]
fn test_solve11121() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
10 10
10 11"
                .to_string(),
            want: "OK
ERROR"
                .to_string(),
        },
        TestData {
            s: "1
1 1"
            .to_string(),
            want: "OK".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve11121(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
