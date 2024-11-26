use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10205(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    let mut ans = Vec::with_capacity(t);

    for case_number in 1..=t {
        let initial_hairs = read_value(read_line(reader));
        let actions = read_line(reader);
        let remaining_hairs = calculate_remaining_hairs(initial_hairs, &actions);
        ans.push(format_result(case_number, remaining_hairs));
    }

    write!(writer, "{}", ans.join("\n\n")).unwrap();
}

fn calculate_remaining_hairs(mut hairs: usize, actions: &str) -> usize {
    for action in actions.chars() {
        match action {
            'c' => hairs += 1,
            'b' => {
                hairs = hairs.saturating_sub(1);
            }
            _ => unreachable!("invalid action"),
        }
    }
    hairs
}

fn format_result(case_number: usize, hairs: usize) -> String {
    format!("Data Set {}:\n{}", case_number, hairs)
}

// https://www.acmicpc.net/problem/10205
// noinspection SpellCheckingInspection
// 헤라클레스와 히드라
#[test]
fn test_solve10205() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
10
cbbbbccbb
10
bbbbbbbbbccbbb"
                .to_string(),
            want: "Data Set 1:
7

Data Set 2:
0"
            .to_string(),
        },
        TestData {
            s: "1
1
b"
            .to_string(),
            want: "Data Set 1:
0"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10205(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
