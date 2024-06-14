use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1515(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let sequence = s.as_bytes();

    let ans = find_final_number_in_sequence(sequence);
    write!(writer, "{}", ans).unwrap();
}

fn find_final_number_in_sequence(sequence: &[u8]) -> usize {
    let mut cursor: usize = 0;
    let mut current_number: usize = 0;

    while cursor < sequence.len() {
        current_number += 1;
        let number_str = current_number.to_string();
        for &digit_byte in number_str.as_bytes() {
            if sequence[cursor] == digit_byte {
                cursor += 1;
                if cursor == sequence.len() {
                    break;
                }
            }
        }
    }
    current_number
}

// https://www.acmicpc.net/problem/1
// 수 이어 쓰기
#[test]
fn test_solve() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1234".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "234092".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "999909".to_string(),
            want: "49".to_string(),
        },
        TestData {
            s: "82340329923".to_string(),
            want: "43".to_string(),
        },
        TestData {
            s: "32098221".to_string(),
            want: "61".to_string(),
        },
        TestData {
            s: "1111111".to_string(),
            want: "14".to_string(),
        },
        TestData {
            s: "00000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            want: "400".to_string(),
        },
        TestData {
            s: "345029834023049820394802334909240982039842039483294792934790209".to_string(),
            want: "279".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1515(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
