use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13419(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    let mut answers = Vec::with_capacity(num_cases);
    for _ in 0..num_cases {
        let sequence = read_line(reader);

        let mut first_sequence = String::new();
        let mut second_sequence = String::new();

        let mut has_first_cycled = false;
        let mut has_second_cycled = false;

        for (index, char) in sequence.chars().cycle().enumerate() {
            let is_even_index = index % 2 == 0;
            if is_even_index {
                if let Some(first_char) = first_sequence.chars().next() {
                    has_first_cycled |= char == first_char;
                }
                if !has_first_cycled {
                    first_sequence.push(char);
                }
            } else {
                if let Some(second_char) = second_sequence.chars().next() {
                    has_second_cycled |= char == second_char;
                }
                if !has_second_cycled {
                    second_sequence.push(char);
                }
            }

            if has_first_cycled && has_second_cycled {
                break;
            }
        }

        answers.push(format!("{}\n{}", first_sequence, second_sequence));
    }

    writeln!(writer, "{}", answers.join("\n")).unwrap();
}

// https://www.acmicpc.net/problem/13419
// noinspection SpellCheckingInspection
// 탕수육
#[test]
fn test_solve13419() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
ABC
ABCFXZ
K
DY"
            .to_string(),
            want: "ACB
BAC
ACX
BFZ
K
K
D
Y"
            .to_string(),
        },
        TestData {
            s: "1
X"
            .to_string(),
            want: "X
X"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13419(&mut reader, &mut writer);

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
